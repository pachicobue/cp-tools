#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to spawn(command: `$ {0}`).")]
    SpawnFailed(Command),

    // Generally, exit statuses "Aborted" or "Timeout" are not treated as errors in this application.
    // To get errors reported, please specify `ensure_success=true` when calling the relevant function or method.
    #[error("Program aborted(command:`$ {0}`, stderr:`{1}`).")]
    ProgramAborted(Command, String),
    #[error("Program timeout(command:`$ {0}`, elapsed:{2}ms/{1}ms")]
    ProgramTimeout(Command, u64, u64),
}

#[derive(Debug, Clone)]
pub struct Status {
    pub summary: StatusSummary,
    pub detail: StatusDetail,
}
impl Status {
    pub fn from(output: std::process::Output) -> Self {
        let start = tokio::time::Instant::now();
        let detail = StatusDetail {
            stdout: String::from_utf8_lossy(&output.stdout).into(),
            stderr: String::from_utf8_lossy(&output.stderr).into(),
            elapsed_ms: (tokio::time::Instant::now() - start).as_millis() as u64,
        };
        Status {
            summary: if output.status.success() {
                StatusSummary::Success
            } else {
                StatusSummary::Aborted
            },
            detail,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatusSummary {
    Success,
    Aborted,
    Timeout,
}

#[derive(Debug, Clone)]
pub struct StatusDetail {
    // To capture stdout/stderr, it is necessary to specify `Stdio::piped()` for `CommandIoRedirection::stdout/stderr`.
    // Otherwise an empty string will return.
    pub stdout: String,
    pub stderr: String,
    pub elapsed_ms: u64,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
}
impl Command {
    pub fn new<S1, I1, S2>(program: S1, args: I1) -> Self
    where
        S1: AsRef<str>,
        I1: IntoIterator<Item = S2>,
        S2: AsRef<str>,
    {
        Command {
            program: program.as_ref().to_string(),
            args: args
                .into_iter()
                .map(|arg| arg.as_ref().to_string())
                .collect(),
        }
    }
    pub fn spawn(&self, redirect: IoRedirection) -> Result<tokio::process::Child, Error> {
        let mut command = tokio::process::Command::new(&self.program);
        command
            .kill_on_drop(true)
            .args(&self.args)
            .stdin(redirect.stdin)
            .stdout(redirect.stdout)
            .stderr(redirect.stderr)
            .spawn()
            .map_err(|_| Error::SpawnFailed(self.to_owned()))
    }
    pub fn exec(
        &self,
        redirect: IoRedirection,
        timeout_ms: u64,
        ensure_success: bool,
    ) -> Result<Status, Error> {
        let res = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                match tokio::time::timeout(
                    tokio::time::Duration::from_millis(timeout_ms) * 2,
                    async {
                        Ok(Status::from(
                            self.spawn(redirect)?.wait_with_output().await.unwrap(),
                        ))
                    },
                )
                .await
                {
                    Ok(Err(e)) => Err(e),
                    Ok(Ok(Status { summary, detail })) => match summary {
                        StatusSummary::Success => {
                            if detail.elapsed_ms <= timeout_ms {
                                Ok(Status {
                                    summary: StatusSummary::Success,
                                    detail,
                                })
                            } else {
                                Ok(Status {
                                    summary: StatusSummary::Timeout,
                                    detail,
                                })
                            }
                        }
                        StatusSummary::Aborted => Ok(Status {
                            summary: StatusSummary::Aborted,
                            detail,
                        }),
                        _ => unreachable!(),
                    },
                    Err(_) => Ok(Status {
                        summary: StatusSummary::Timeout,
                        detail: StatusDetail {
                            stdout: "".into(),
                            stderr: "".into(),
                            elapsed_ms: timeout_ms * 2,
                        },
                    }),
                }
            })?;
        if ensure_success {
            match res.summary {
                StatusSummary::Success => Ok(res),
                StatusSummary::Aborted => {
                    Err(Error::ProgramAborted(self.to_owned(), res.detail.stderr))
                }
                StatusSummary::Timeout => Err(Error::ProgramTimeout(
                    self.to_owned(),
                    timeout_ms,
                    res.detail.elapsed_ms,
                )),
            }
        } else {
            Ok(res)
        }
    }
}

impl<T: AsRef<str>> From<T> for Command {
    fn from(command_str: T) -> Self {
        let args: Vec<&str> = command_str.as_ref().split(' ').collect();
        Self::new(args.first().unwrap_or(&""), args[1..].to_owned())
    }
}
impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.program, self.args.join(" "))
    }
}

#[derive(Debug)]
pub struct IoRedirection {
    pub stdin: std::process::Stdio,
    pub stdout: std::process::Stdio,
    pub stderr: std::process::Stdio,
}
