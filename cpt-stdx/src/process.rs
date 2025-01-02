use std::process::Stdio;
use std::time::Duration;

use itertools::Itertools;

use crate::task::{run_task, TaskError};

#[derive(Debug, Clone)]
pub struct CommandResult {
    pub summary: CommandResultSummary,
    pub detail: CommandResultDetail,
}
impl CommandResult {
    pub fn is_success(&self) -> bool {
        self.summary == CommandResultSummary::Success
    }
}
impl From<TaskError> for CommandResult {
    fn from(e: TaskError) -> Self {
        Self {
            summary: CommandResultSummary::Aborted,
            detail: CommandResultDetail {
                stdout: "".into(),
                stderr: e.to_string(),
                elapsed: Duration::default(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandResultSummary {
    Success,
    Aborted,
    Timeout,
}

#[derive(Debug, Clone)]
pub struct CommandResultDetail {
    pub stdout: String,
    pub stderr: String,
    pub elapsed: Duration,
}

#[derive(Debug, Clone)]
pub struct CommandExpression {
    pub program: String,
    pub args: Vec<String>,
}
impl CommandExpression {
    pub fn new<S1, I1, S2>(program: S1, args: I1) -> Self
    where
        S1: AsRef<str>,
        I1: IntoIterator<Item = S2>,
        S2: AsRef<str>,
    {
        CommandExpression {
            program: program.as_ref().to_string(),
            args: args
                .into_iter()
                .map(|arg| arg.as_ref().to_string())
                .collect_vec(),
        }
    }
}
impl std::fmt::Display for CommandExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.program, self.args.iter().join(" "))
    }
}

#[derive(Debug)]
pub struct CommandIoRedirection {
    pub stdin: Stdio,
    pub stdout: Stdio,
    pub stderr: Stdio,
}
impl Default for CommandIoRedirection {
    fn default() -> Self {
        Self {
            stdin: Stdio::piped(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        }
    }
}

pub async fn command_task(
    expr: CommandExpression,
    redirect: CommandIoRedirection,
    timeout: Option<f32>,
) -> CommandResult {
    if let Some(timeout_sec) = timeout {
        command_timeout(expr, redirect, timeout_sec).await
    } else {
        command(expr, redirect).await
    }
}

async fn command(expr: CommandExpression, redirect: CommandIoRedirection) -> CommandResult {
    let mut command = tokio::process::Command::new(&expr.program);

    let start = tokio::time::Instant::now();
    let output = command
        .args(&expr.args)
        .stdin(redirect.stdin)
        .stdout(redirect.stdout)
        .stderr(redirect.stderr)
        .spawn()
        .expect("Command failed to start")
        .wait_with_output()
        .await
        .expect("Command failed to run");
    let detail = CommandResultDetail {
        stdout: String::from_utf8_lossy(&output.stdout).into(),
        stderr: String::from_utf8_lossy(&output.stderr).into(),
        elapsed: tokio::time::Instant::now() - start,
    };
    CommandResult {
        summary: if output.status.success() {
            CommandResultSummary::Success
        } else {
            CommandResultSummary::Aborted
        },
        detail,
    }
}

async fn command_timeout(
    expr: CommandExpression,
    redirect: CommandIoRedirection,
    timeout_sec: f32,
) -> CommandResult {
    let tl = Duration::from_secs_f32(timeout_sec);
    match tokio::time::timeout(tl * 2, command(expr, redirect)).await {
        Ok(CommandResult { summary, detail }) => match summary {
            CommandResultSummary::Success => {
                if detail.elapsed <= tl {
                    CommandResult {
                        summary: CommandResultSummary::Success,
                        detail,
                    }
                } else {
                    CommandResult {
                        summary: CommandResultSummary::Timeout,
                        detail,
                    }
                }
            }
            CommandResultSummary::Aborted => CommandResult {
                summary: CommandResultSummary::Aborted,
                detail,
            },
            _ => unreachable!(),
        },
        Err(_) => CommandResult {
            summary: CommandResultSummary::Timeout,
            detail: CommandResultDetail {
                stdout: "".into(),
                stderr: "".into(),
                elapsed: tl * 2,
            },
        },
    }
}

pub fn run_command_simple(expr: CommandExpression) -> CommandResult {
    run_task(command(expr, CommandIoRedirection::default())).unwrap_or_else(CommandResult::from)
}
