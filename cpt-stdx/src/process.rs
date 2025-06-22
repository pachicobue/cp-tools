/// Error types for process execution operations.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Failed to spawn the command process.
    #[error("Failed to spawn(command: `$ {0}`).")]
    SpawnFailed(Command),

    /// Program exited with non-zero status.
    ///
    /// Generally, exit statuses "Aborted" or "Timeout" are not treated as errors in this application.
    /// To get errors reported, please specify `ensure_success=true` when calling the relevant function or method.
    #[error("Program aborted(command:`$ {0}`, stderr:`{1}`).")]
    ProgramAborted(Command, String),
    /// Program exceeded the specified timeout limit.
    #[error("Program timeout(command:`$ {0}`, elapsed:{2}ms/{1}ms")]
    ProgramTimeout(Command, u64, u64),
}

/// Represents the execution status of a command.
#[derive(Debug, Clone)]
pub struct Status {
    /// High-level summary of the execution result.
    pub summary: StatusSummary,
    /// Detailed information about the execution.
    pub detail: StatusDetail,
}
impl Status {
    /// Creates a Status from a std::process::Output.
    ///
    /// # Arguments
    ///
    /// * `output` - The process output to convert
    ///
    /// # Returns
    ///
    /// A Status instance with appropriate summary and details
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

/// High-level summary of command execution status.
#[derive(Debug, Clone, PartialEq)]
pub enum StatusSummary {
    /// Command completed successfully (exit code 0).
    Success,
    /// Command terminated with non-zero exit code.
    Aborted,
    /// Command exceeded the specified timeout.
    Timeout,
}

/// Detailed information about command execution.
#[derive(Debug, Clone)]
pub struct StatusDetail {
    /// Standard output captured from the command.
    ///
    /// To capture stdout/stderr, it is necessary to specify `Stdio::piped()` for `CommandIoRedirection::stdout/stderr`.
    /// Otherwise an empty string will return.
    pub stdout: String,
    /// Standard error captured from the command.
    pub stderr: String,
    /// Execution time in milliseconds.
    pub elapsed_ms: u64,
}

/// Represents a command to be executed with its program name and arguments.
#[derive(Debug, Clone)]
pub struct Command {
    /// The program name or path to execute.
    pub program: String,
    /// The command-line arguments to pass to the program.
    pub args: Vec<String>,
}
impl Command {
    /// Creates a new Command with the specified program and arguments.
    ///
    /// # Arguments
    ///
    /// * `program` - The program name or path to execute
    /// * `args` - An iterable of command-line arguments
    ///
    /// # Returns
    ///
    /// A new Command instance
    ///
    /// # Example
    ///
    /// ```rust
    /// use cpt_stdx::process::Command;
    ///
    /// let cmd = Command::new("echo", vec!["hello", "world"]);
    /// assert_eq!(cmd.program, "echo");
    /// assert_eq!(cmd.args, vec!["hello", "world"]);
    /// ```
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
    /// Spawns the command as a child process with the specified I/O redirection.
    ///
    /// # Arguments
    ///
    /// * `redirect` - I/O redirection configuration for stdin, stdout, and stderr
    ///
    /// # Returns
    ///
    /// * `Ok(Child)` - The spawned child process
    /// * `Err(Error::SpawnFailed)` - If the process could not be spawned
    ///
    /// # Example
    ///
    /// ```rust
    /// use cpt_stdx::process::{Command, IoRedirection};
    /// use std::process::Stdio;
    ///
    /// let cmd = Command::new("echo", vec!["hello"]);
    /// let redirect = IoRedirection {
    ///     stdin: Stdio::null(),
    ///     stdout: Stdio::piped(),
    ///     stderr: Stdio::piped(),
    /// };
    /// // let child = cmd.spawn(redirect).expect("Failed to spawn");
    /// ```
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
    /// Executes the command and waits for it to complete.
    ///
    /// # Arguments
    ///
    /// * `redirect` - I/O redirection configuration
    /// * `timeout_ms` - Maximum execution time in milliseconds
    /// * `ensure_success` - If true, returns an error for non-zero exit codes or timeouts
    ///
    /// # Returns
    ///
    /// * `Ok(Status)` - Execution status and details
    /// * `Err(Error::SpawnFailed)` - If the process could not be spawned
    /// * `Err(Error::ProgramAborted)` - If ensure_success is true and the program failed
    /// * `Err(Error::ProgramTimeout)` - If ensure_success is true and the program timed out
    ///
    /// # Example
    ///
    /// ```rust
    /// use cpt_stdx::process::{Command, IoRedirection};
    /// use std::process::Stdio;
    ///
    /// let cmd = Command::new("echo", vec!["hello"]);
    /// let redirect = IoRedirection {
    ///     stdin: Stdio::null(),
    ///     stdout: Stdio::piped(),
    ///     stderr: Stdio::piped(),
    /// };
    /// let status = cmd.exec(redirect, 5000, false).expect("Failed to execute");
    /// ```
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
    /// Creates a Command from a string by splitting on spaces.
    ///
    /// # Arguments
    ///
    /// * `command_str` - A string containing the program and arguments separated by spaces
    ///
    /// # Returns
    ///
    /// A Command instance with the first word as program and remaining words as arguments
    ///
    /// # Example
    ///
    /// ```rust
    /// use cpt_stdx::process::Command;
    ///
    /// let cmd: Command = "echo hello world".into();
    /// assert_eq!(cmd.program, "echo");
    /// assert_eq!(cmd.args, vec!["hello", "world"]);
    /// ```
    fn from(command_str: T) -> Self {
        let args: Vec<&str> = command_str.as_ref().split(' ').collect();
        Self::new(args.first().unwrap_or(&""), args[1..].to_owned())
    }
}
impl std::fmt::Display for Command {
    /// Formats the Command as a space-separated string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cpt_stdx::process::Command;
    ///
    /// let cmd = Command::new("echo", vec!["hello", "world"]);
    /// assert_eq!(format!("{}", cmd), "echo hello world");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.program, self.args.join(" "))
    }
}

/// Configuration for I/O redirection when spawning processes.
#[derive(Debug)]
pub struct IoRedirection {
    /// Standard input redirection (null, piped, inherit).
    pub stdin: std::process::Stdio,
    /// Standard output redirection (null, piped, inherit).
    pub stdout: std::process::Stdio,
    /// Standard error redirection (null, piped, inherit).
    pub stderr: std::process::Stdio,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(unix)]
    use std::os::unix::process::ExitStatusExt;
    use std::process::Stdio;

    #[test]
    fn test_command_new() {
        let cmd = Command::new("echo", vec!["hello", "world"]);
        assert_eq!(cmd.program, "echo");
        assert_eq!(cmd.args, vec!["hello", "world"]);
    }

    #[test]
    fn test_command_from_string() {
        let cmd: Command = "echo hello world".into();
        assert_eq!(cmd.program, "echo");
        assert_eq!(cmd.args, vec!["hello", "world"]);
    }

    #[test]
    fn test_command_from_empty_string() {
        let cmd: Command = "".into();
        assert_eq!(cmd.program, "");
        assert_eq!(cmd.args, Vec::<String>::new());
    }

    #[test]
    fn test_command_from_single_word() {
        let cmd: Command = "echo".into();
        assert_eq!(cmd.program, "echo");
        assert_eq!(cmd.args, Vec::<String>::new());
    }

    #[test]
    fn test_command_display() {
        let cmd = Command::new("echo", vec!["hello", "world"]);
        assert_eq!(format!("{}", cmd), "echo hello world");
    }

    #[test]
    fn test_command_display_no_args() {
        let cmd = Command::new("echo", Vec::<String>::new());
        assert_eq!(format!("{}", cmd), "echo ");
    }

    #[test]
    #[cfg(unix)]
    fn test_status_from_success() {
        let output = std::process::Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: b"hello".to_vec(),
            stderr: b"".to_vec(),
        };
        let status = Status::from(output);
        assert_eq!(status.summary, StatusSummary::Success);
        assert_eq!(status.detail.stdout, "hello");
        assert_eq!(status.detail.stderr, "");
    }

    #[test]
    #[cfg(unix)]
    fn test_status_from_failure() {
        let output = std::process::Output {
            status: std::process::ExitStatus::from_raw(256), // Exit code 1
            stdout: b"".to_vec(),
            stderr: b"error".to_vec(),
        };
        let status = Status::from(output);
        assert_eq!(status.summary, StatusSummary::Aborted);
        assert_eq!(status.detail.stdout, "");
        assert_eq!(status.detail.stderr, "error");
    }

    #[test]
    fn test_command_exec_success() {
        let cmd = Command::new("echo", vec!["hello"]);
        let redirect = IoRedirection {
            stdin: Stdio::null(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        };

        let result = cmd.exec(redirect, 5000, false);
        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status.summary, StatusSummary::Success);
        assert!(status.detail.stdout.contains("hello"));
    }

    #[test]
    fn test_command_exec_with_ensure_success_true() {
        let cmd = Command::new("echo", vec!["hello"]);
        let redirect = IoRedirection {
            stdin: Stdio::null(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        };

        let result = cmd.exec(redirect, 5000, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_command_exec_nonexistent_program() {
        let cmd = Command::new("nonexistent_program_xyz", Vec::<String>::new());
        let redirect = IoRedirection {
            stdin: Stdio::null(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        };

        let result = cmd.exec(redirect, 5000, false);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::SpawnFailed(_) => {}
            _ => panic!("Expected SpawnFailed error"),
        }
    }

    #[test]
    fn test_command_exec_false_command() {
        let cmd = Command::new("false", Vec::<String>::new());
        let redirect = IoRedirection {
            stdin: Stdio::null(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        };

        let result = cmd.exec(redirect, 5000, false);
        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status.summary, StatusSummary::Aborted);
    }

    #[test]
    fn test_command_exec_false_command_with_ensure_success() {
        let cmd = Command::new("false", Vec::<String>::new());
        let redirect = IoRedirection {
            stdin: Stdio::null(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        };

        let result = cmd.exec(redirect, 5000, true);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::ProgramAborted(_, _) => {}
            _ => panic!("Expected ProgramAborted error"),
        }
    }

    #[test]
    fn test_command_exec_timeout() {
        let cmd = Command::new("sleep", vec!["2"]);
        let redirect = IoRedirection {
            stdin: Stdio::null(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        };

        let result = cmd.exec(redirect, 100, false); // 100ms timeout
        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status.summary, StatusSummary::Timeout);
    }

    #[test]
    fn test_command_exec_timeout_with_ensure_success() {
        let cmd = Command::new("sleep", vec!["2"]);
        let redirect = IoRedirection {
            stdin: Stdio::null(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        };

        let result = cmd.exec(redirect, 100, true); // 100ms timeout
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::ProgramTimeout(_, _, _) => {}
            _ => panic!("Expected ProgramTimeout error"),
        }
    }

    #[test]
    fn test_status_summary_equality() {
        assert_eq!(StatusSummary::Success, StatusSummary::Success);
        assert_eq!(StatusSummary::Aborted, StatusSummary::Aborted);
        assert_eq!(StatusSummary::Timeout, StatusSummary::Timeout);
        assert_ne!(StatusSummary::Success, StatusSummary::Aborted);
    }

    #[test]
    fn test_error_display() {
        let cmd = Command::new("test", vec!["arg"]);
        let err = Error::SpawnFailed(cmd.clone());
        assert!(format!("{}", err).contains("Failed to spawn"));
        assert!(format!("{}", err).contains("test arg"));

        let err = Error::ProgramAborted(cmd.clone(), "stderr output".to_string());
        assert!(format!("{}", err).contains("Program aborted"));
        assert!(format!("{}", err).contains("stderr output"));

        let err = Error::ProgramTimeout(cmd, 1000, 2000);
        assert!(format!("{}", err).contains("Program timeout"));
        assert!(format!("{}", err).contains("2000ms/1000ms"));
    }
}
