use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Failed to execute program.")]
    Program(#[source] cpt_stdx::process::Error),
    #[error("Failed to execute judge.")]
    Judge(#[source] cpt_stdx::process::Error),
    #[error("Communication failed.")]
    Communication(#[source] tokio::io::Error),
}

#[derive(Debug, Clone)]
struct JudgeFiles {
    casename: String,
    input: PathBuf,
    debug: PathBuf,
    judge: PathBuf,
}
impl JudgeFiles {
    fn new(testcase: crate::testcase::Testcase, dir: &Path) -> Self {
        Self {
            casename: testcase.casename.to_owned(),
            input: testcase.input,
            debug: dir.join(testcase.casename.to_owned() + ".debug.txt"),
            judge: dir.join(testcase.casename + ".judge.txt"),
        }
    }
}

pub(crate) fn judge(
    program_command: &str,
    judge_command: &str,
    testcase: crate::testcase::Testcase,
    timelimit_ms: u64,
    dir: &Path,
) -> Result<crate::judge::Verdict, Error> {
    use cpt_stdx::fs;
    use cpt_stdx::process::{Status, StatusSummary};

    use crate::judge::Verdict;

    let judge_files = JudgeFiles::new(testcase, dir);
    log::info!("[Judge][{}] Start", judge_files.casename);

    let Status { summary, detail } = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            exec_timeout(
                program_command,
                judge_command,
                judge_files.to_owned(),
                timelimit_ms,
            )
            .await
        })?;

    Ok(match summary {
        StatusSummary::Success => {
            log::info!("[Judge][{}] AC", judge_files.casename);
            Verdict::Ac
        }
        StatusSummary::Aborted => {
            log::warn!("[Judge][{}] WA", judge_files.casename);
            log::warn!("{}", fs::read(&judge_files.debug).unwrap());
            Verdict::Re
        }
        StatusSummary::Timeout => {
            log::warn!(
                "[Judge][{}] TLE ({}ms/{}ms)",
                judge_files.casename,
                detail.elapsed_ms,
                timelimit_ms
            );
            Verdict::Tle
        }
    })
}

async fn exec_timeout(
    program_command: &str,
    judge_command: &str,
    judge_files: JudgeFiles,
    timelimit_ms: u64,
) -> Result<cpt_stdx::process::Status, Error> {
    use std::process::Stdio;

    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    use cpt_stdx::fs;
    use cpt_stdx::process::{Command, IoRedirection, Status, StatusDetail, StatusSummary};

    let result = tokio::time::timeout(
        tokio::time::Duration::from_millis(timelimit_ms) * 2,
        async move {
            let mut program = Command::from(program_command)
                .spawn(IoRedirection {
                    stdin: Stdio::piped(),
                    stdout: Stdio::piped(),
                    stderr: Stdio::from(fs::create(&judge_files.debug, true).unwrap()),
                })
                .map_err(Error::Program)?;
            let mut judge_command = Command::from(judge_command);
            judge_command.args.push(format!("{}",judge_files.input.display()));
            let mut judge = judge_command
                .spawn(IoRedirection {
                    stdin: Stdio::piped(),
                    stdout: Stdio::piped(),
                    stderr: Stdio::from(fs::create(&judge_files.judge, true).unwrap()),
                })
                .map_err(Error::Judge)?;
                let mut program_stdin = program.stdin.take().unwrap();
                let mut program_stdout = BufReader::new(program.stdout.take().unwrap()).lines();
                let mut judge_stdin = judge.stdin.take().unwrap();
                let mut judge_stdout = BufReader::new(judge.stdout.take().unwrap()).lines();
            let interaction = async move {
                loop {
                    tokio::select! {
                        program_line = program_stdout.next_line() => {
                            if let Ok(Some(line)) = program_line {
                                    judge_stdin.write_all(line.as_bytes()).await.map_err(Error::Communication)?;
                                    judge_stdin.write_all(b"\n").await.map_err(Error::Communication)?;
                                    judge_stdin.flush().await.map_err(Error::Communication)?;
                            } else {
                                break;
                            }
                        }
                        judge_line = judge_stdout.next_line() => {
                            if let Ok(Some(line)) = judge_line {
                                    program_stdin.write_all(line.as_bytes()).await.map_err(Error::Communication)?;
                                    program_stdin.write_all(b"\n").await.map_err(Error::Communication)?;
                                    program_stdin.flush().await.map_err(Error::Communication)?;
                            } else {
                                break;
                            }
                        }
                    }
                }
                Ok::<(), Error>(())
            };
            tokio::spawn(interaction);
            Ok(Status::from(judge.wait_with_output().await.unwrap()))
        },
    ).await;
    match result {
        Ok(Err(e)) => Err(e),
        Ok(Ok(Status { summary, detail })) => match summary {
            StatusSummary::Success => {
                if detail.elapsed_ms <= timelimit_ms {
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
                elapsed_ms: timelimit_ms * 2,
            },
        }),
    }
}
