use std::{
    ffi::{OsStr, OsString},
    process::Stdio,
    time::Duration,
};

use color_eyre::eyre::{eyre, Context, Result};
use itertools::Itertools;
use tempfile;
use tokio;

use crate::{core::task::run_task, tempfile_builder};

#[derive(Debug, Clone)]
pub(crate) enum CommandResult {
    Success(CommandResultDetail),
    Aborted(CommandResultDetail),
    Timeout(CommandResultDetail),
}
impl CommandResult {
    pub(crate) fn detail_of_success(&self) -> Result<CommandResultDetail> {
        match self {
            Self::Success(detail) => Ok(detail.clone()),
            Self::Aborted(_) => Err(eyre!("Aborted")),
            Self::Timeout(_) => Err(eyre!("Timeout")),
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
                .map(|e| e.as_ref().to_os_string())
                .collect_vec(),
        }
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
) -> Result<CommandResult> {
    if let Some(timeout_sec) = timeout {
        command_timeout(expr, redirect, timeout_sec).await
    } else {
        command(expr, redirect).await
    }
}

async fn command(expr: CommandExpression, redirect: CommandIoRedirection) -> Result<CommandResult> {
    let mut command = tokio::process::Command::new(&expr.program);

    let start = tokio::time::Instant::now();
    let output = command
        .args(&expr.args)
        .stdin(redirect.stdin)
        .stdout(redirect.stdout)
        .stderr(redirect.stderr)
        .spawn()
        .wrap_err("Failed to spawn child process.")?
        .wait_with_output()
        .await
        .wrap_err("Failed to get output from process.")?;
    let detail = CommandResultDetail {
        stdout: String::from_utf8_lossy(&output.stdout).into(),
        stderr: String::from_utf8_lossy(&output.stderr).into(),
        elapsed: tokio::time::Instant::now() - start,
    };
    if output.status.success() {
        Ok(CommandResult::Success(detail))
    } else {
        Ok(CommandResult::Aborted(detail))
    }
}

async fn command_timeout(
    expr: CommandExpression,
    redirect: CommandIoRedirection,
    timeout_sec: f32,
) -> Result<CommandResult> {
    let tl = Duration::from_secs_f32(timeout_sec);
    match tokio::time::timeout(tl * 2, command(expr, redirect)).await {
        Ok(wrapped_result) => match wrapped_result? {
            CommandResult::Success(detail) => {
                if detail.elapsed <= tl {
                    Ok(CommandResult::Success(detail))
                } else {
                    Ok(CommandResult::Timeout(detail))
                }
            }
            CommandResult::Aborted(detail) => Ok(CommandResult::Aborted(detail)),
            _ => unreachable!(),
        },
        Err(_) => Ok(CommandResult::Timeout(CommandResultDetail {
            stdout: "".into(),
            stderr: "".into(),
            elapsed: tl * 2,
        })),
    }
}

pub(crate) fn run_command_simple(expr: CommandExpression) -> Result<CommandResult> {
    log::info!(
        "$ {} {}",
        &expr.program.to_string_lossy(),
        &expr.args.iter().map(|arg| arg.to_string_lossy()).join(" ")
    );
    let tempdir = tempfile_builder!();
    let result = run_task(command(
        expr,
        CommandIoRedirection {
            stdin: Stdio::piped(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        },
    ))?;
    tempdir.close().wrap_err("Failed to delete tempdir.")?;
    describe_result(&result);
    Ok(result)
}

fn describe_result(result: &CommandResult) {
    match result {
        CommandResult::Success(detail) => {
            log::info!(
                "Command successfully done: {}ms elapsed.",
                detail.elapsed.as_millis()
            );
            log::trace!("{}", detail.stdout);
        }
        CommandResult::Aborted(detail) => {
            log::error!("Command aborted: {}ms elapsed.", detail.elapsed.as_millis());
            log::error!("{}", detail.stderr)
        }
        CommandResult::Timeout(detail) => {
            log::error!("Command timeout: {}ms elapsed.", detail.elapsed.as_millis())
        }
    }
}
