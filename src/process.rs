use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    time::Duration,
};

use color_eyre::eyre::{eyre, Context, Result};
use itertools::Itertools;
use tempfile;
use tokio;

use crate::{
    config::metadata::crate_name,
    fs::{create_async, open_async, read_async},
    printer::abbr,
    styled,
    task::{run_multi_task, run_single_task, TaskResult},
};

#[derive(Debug, Clone)]
pub(crate) struct CmdExpression {
    pub command: OsString,
    pub args: Vec<OsString>,
}
impl CmdExpression {
    pub(crate) fn new(
        command: impl AsRef<OsStr>,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> Self {
        CmdExpression {
            command: command.as_ref().into(),
            args: args.into_iter().map(|e| e.as_ref().into()).collect_vec(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct CmdIoRedirection {
    pub stdin: Option<PathBuf>,
    pub stdout: Option<PathBuf>,
    pub stderr: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub(crate) struct CmdOutput {
    pub stdout: String,
    pub stderr: String,
}

pub(crate) fn run_single(
    command: CmdExpression,
    timeout_sec: Option<f32>,
    io_redirection: CmdIoRedirection,
) -> Result<TaskResult<CmdOutput>> {
    run_multiple(command, timeout_sec, vec![io_redirection]).map(|results| results[0].clone())
}

pub(crate) fn run_multiple(
    command: CmdExpression,
    timeout_sec: Option<f32>,
    io_redirections: Vec<CmdIoRedirection>,
) -> Result<Vec<TaskResult<CmdOutput>>> {
    log::info!(
        "$ {} {}",
        &command.command.to_string_lossy(),
        &command.args.iter().map(|e| e.to_string_lossy()).join(" "),
    );
    for (i, red) in io_redirections.iter().enumerate() {
        log::debug!(
            "[{}] stdin: `{:?}`, stdout: `{:?}`, stderr: `{:?}`",
            i,
            red.stdin,
            red.stdout,
            red.stderr
        );
    }

    let tempdir = tempfile::Builder::new()
        .prefix(&format!("{}-", crate_name()))
        .tempdir()
        .context("Failed to create tempdir.")?;
    let temppath = tempdir.path();
    let tasks = io_redirections
        .into_iter()
        .enumerate()
        .map(|(i, red)| {
            let stdin = red.stdin.clone().unwrap_or(default_stdin());
            let stdout = red.stdout.clone().unwrap_or(default_stdout(&temppath, i));
            let stderr = red.stderr.clone().unwrap_or(default_stderr(&temppath, i));
            exec(command.clone(), stdin, stdout, stderr)
        })
        .collect_vec();
    let results = run_multi_task(tasks, timeout_sec);
    tempdir.close().context("Failed to close tempdir.")?;
    results
}

async fn exec(
    cmd: CmdExpression,
    stdin: PathBuf,
    stdout: PathBuf,
    stderr: PathBuf,
) -> Result<TaskResult<CmdOutput>> {
    let stdin_file = open_async(&stdin).await?.into_std().await;
    let stdout_file = create_async(&stdout).await?.into_std().await;
    let stderr_file = create_async(&stderr).await?.into_std().await;
    let output = tokio::process::Command::new(&cmd.command)
        .args(&cmd.args)
        .stdin(stdin_file)
        .stdout(stdout_file)
        .stderr(stderr_file)
        .kill_on_drop(true)
        .spawn()
        .context("Failed to spawn child process.")?
        .wait_with_output()
        .await
        .context("Failed to get output from child process.")?;
    let cmd_output = CmdOutput {
        stdout: read_async(stdout).await?,
        stderr: read_async(stderr).await?,
    };
    if output.status.success() {
        Ok(TaskResult::Done(cmd_output))
    } else {
        Ok(TaskResult::RuntimeError(cmd_output))
    }
}

fn default_stdin() -> PathBuf {
    Path::new("/dev/null").to_path_buf()
}
fn default_stdout(basedir: &Path, index: usize) -> PathBuf {
    basedir.join(&format!("stdout-{}", index))
}
fn default_stderr(basedir: &Path, index: usize) -> PathBuf {
    basedir.join(&format!("stderr-{}", index))
}

const fn default_timeout_sec() -> f32 {
    30.
}
