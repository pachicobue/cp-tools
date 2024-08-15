use std::{ffi::OsString, path::PathBuf, process::Stdio};

use clap::{Args, ValueHint};
use color_eyre::eyre::{ensure, Context, Result};
use itertools::Itertools;
use tempfile;

use crate::{
    core::{
        fs::read_async,
        judge::{collect_judge_paths, JudgePaths, Verdict},
        process::{command_task, CommandExpression, CommandIoRedirection, CommandResult},
        task::run_tasks,
    },
    styled, tempfile_builder,
};

#[derive(Args, Debug)]
pub(crate) struct BatchArgs {
    /// 実行コマンド
    #[arg(short = 'c')]
    command: String,

    /// テストディレクトリ
    #[arg(short = 'd', alias = "dir", value_hint(ValueHint::FilePath))]
    directory: PathBuf,

    /// TL(秒単位)
    #[arg(short = 't')]
    timelimit: Option<f32>,
}

pub(crate) fn batch(args: &BatchArgs) -> Result<Vec<Verdict>> {
    log::info!("{}\n{:?}", styled!("Batch Test").bold().green(), args);
    check_args(args)?;

    let tempdir = tempfile_builder!();
    let temppath = tempdir.path();

    let judge_paths = collect_judge_paths(&args.directory, temppath);
    log::debug!("judge_paths: {:#?}", &judge_paths);
    if judge_paths.is_empty() {
        log::error!("No testcase found!");
    }
    let check_tasks = judge_paths
        .into_iter()
        .map(|judge_path| judge_single(args.command.clone(), judge_path, args.timelimit))
        .collect_vec();
    let verdicts = run_tasks(check_tasks)?;
    tempdir.close().wrap_err("Failed to close tempdir.")?;
    Ok(verdicts)
}

async fn judge_single(command: String, judge_path: JudgePaths, tl: Option<f32>) -> Result<Verdict> {
    let sol_result = command_task(
        CommandExpression::new(command, &Vec::<OsString>::new()),
        CommandIoRedirection {
            stdin: Stdio::piped(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        },
        tl,
    )
    .await?;
    match sol_result {
        CommandResult::Success(detail) => {
            if let Some(expect_path) = judge_path.expect {
                let actual = detail.stdout;
                let expect = read_async(&expect_path).await?;
                Ok(if actual == expect {
                    Verdict::Ac
                } else {
                    Verdict::Wa
                })
            } else {
                Ok(Verdict::Ac)
            }
        }
        CommandResult::Aborted(_) => Ok(Verdict::Re),
        CommandResult::Timeout(_) => Ok(Verdict::Tle),
    }
}

fn check_args(args: &BatchArgs) -> Result<()> {
    ensure!(
        &args.directory.exists(),
        "`{}` not found.",
        &args.directory.display()
    );
    ensure!(
        &args.directory.is_dir(),
        "`{}` is not directory.",
        &args.directory.display()
    );
    Ok(())
}
