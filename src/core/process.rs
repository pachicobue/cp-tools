use std::{
    ffi::{OsStr, OsString},
    process::Stdio,
    time::Duration,
};

use itertools::Itertools;
use strum;
use tokio;

use crate::core::{printer::abbr, task::run_task};

#[derive(Debug, Clone, strum::EnumIs)]
pub(crate) enum CommandResult {
    Success(CommandResultDetail),
    Aborted(CommandResultDetail),
    Timeout(CommandResultDetail),
}
impl CommandResult {
    pub(crate) fn get_detail(&self) -> &CommandResultDetail {
        match self {
            Self::Success(detail) => detail,
            Self::Aborted(detail) => detail,
            Self::Timeout(detail) => detail,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CommandResultDetail {
    pub stdout: String,
    pub stderr: String,
    pub elapsed: Duration,
}

#[derive(Debug, Clone)]
pub(crate) struct CommandExpression {
    pub program: OsString,
    pub args: Vec<OsString>,
}
impl CommandExpression {
    pub(crate) fn new<S1, I1, S2>(program: S1, args: I1) -> Self
    where
        S1: AsRef<OsStr>,
        I1: IntoIterator<Item = S2>,
        S2: AsRef<OsStr>,
    {
        CommandExpression {
            program: program.as_ref().to_os_string(),
            args: args
                .into_iter()
                .map(|arg| arg.as_ref().to_os_string())
                .collect_vec(),
        }
    }

    pub(crate) fn to_string(&self) -> String {
        format!(
            "{} {}",
            self.program.to_string_lossy(),
            self.args.iter().map(|arg| arg.to_string_lossy()).join(" ")
        )
    }
}

#[derive(Debug)]
pub(crate) struct CommandIoRedirection {
    pub stdin: Stdio,
    pub stdout: Stdio,
    pub stderr: Stdio,
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
    if output.status.success() {
        CommandResult::Success(detail)
    } else {
        CommandResult::Aborted(detail)
    }
}

async fn command_timeout(
    expr: CommandExpression,
    redirect: CommandIoRedirection,
    timeout_sec: f32,
) -> CommandResult {
    let tl = Duration::from_secs_f32(timeout_sec);
    match tokio::time::timeout(tl * 2, command(expr, redirect)).await {
        Ok(wrapped_result) => match wrapped_result {
            CommandResult::Success(detail) => {
                if detail.elapsed <= tl {
                    CommandResult::Success(detail)
                } else {
                    CommandResult::Timeout(detail)
                }
            }
            CommandResult::Aborted(detail) => CommandResult::Aborted(detail),
            _ => unreachable!(),
        },
        Err(_) => CommandResult::Timeout(CommandResultDetail {
            stdout: "".into(),
            stderr: "".into(),
            elapsed: tl * 2,
        }),
    }
}

pub(crate) fn run_command_simple(expr: CommandExpression) -> CommandResult {
    log::info!("$ {}", expr.to_string());
    let result = run_task(command(
        expr,
        CommandIoRedirection {
            stdin: Stdio::piped(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        },
    ));
    describe_result(&result);
    result
}

fn describe_result(result: &CommandResult) {
    match result {
        CommandResult::Success(detail) => {
            log::info!(
                "Command successfully done: {}ms elapsed.",
                detail.elapsed.as_millis()
            );
            log::debug!("stderr:\n{}", &detail.stderr);
            log::debug!("stdout:\n{}", abbr(&detail.stdout));
            log::trace!("stdout:\n{}", &detail.stdout);
        }
        CommandResult::Aborted(detail) => {
            log::error!("Command aborted: {}ms elapsed.", detail.elapsed.as_millis());
            log::error!("stderr:\n{}", &detail.stderr);
            log::debug!("stdout:\n{}", abbr(&detail.stdout));
            log::trace!("stdout:\n{}", &detail.stdout);
        }
        CommandResult::Timeout(detail) => {
            log::error!("Command timeout: {}ms elapsed.", detail.elapsed.as_millis())
        }
    }
}
