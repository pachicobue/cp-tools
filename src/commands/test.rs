use std::{ffi::OsString, path::PathBuf, process::Stdio};

use clap::{Args, ValueHint};
use itertools::Itertools;

use crate::{
    core::{
        error::{TestArgumentError, TestCommandError},
        fs::{read_async, with_tempdir},
        judge::{collect_judge_paths, JudgePaths, Verdict},
        process::{command_task, CommandExpression, CommandIoRedirection, CommandResult},
        task::run_tasks,
    },
    styled,
};

#[derive(Args, Debug)]
pub(crate) struct TestArgs {
    /// 実行コマンド
    #[arg(short = 'c')]
    command: String,

    /// テストディレクトリ
    #[arg(short = 'd', visible_alias = "dir", value_hint(ValueHint::FilePath))]
    directory: PathBuf,

    /// TL(秒単位)
    #[arg(short = 't', visible_alias = "tl")]
    timelimit: Option<f32>,
}

pub(crate) fn test(args: &TestArgs) -> Result<Vec<Verdict>, TestCommandError> {
    log::info!("{}\n{:?}", styled!("Batch Test").bold().green(), args);
    check_args(args)?;

    let verdicts = with_tempdir(|tempdir| -> Result<Vec<Verdict>, TestCommandError> {
        let temppath = tempdir.path();
        let judge_paths = collect_judge_paths(&args.directory, temppath);
        if judge_paths.is_empty() {
            return Err(TestCommandError::TestCaseNotFound);
        }
        let check_tasks = judge_paths
            .into_iter()
            .map(|judge_path| judge_single(args.command.clone(), judge_path, args.timelimit))
            .collect_vec();
        Ok(run_tasks(check_tasks))
    })?;
    Ok(verdicts)
}

async fn judge_single(command: String, judge_path: JudgePaths, tl: Option<f32>) -> Verdict {
    let sol_result = command_task(
        CommandExpression::new(command, Vec::<OsString>::new()),
        CommandIoRedirection {
            stdin: Stdio::piped(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        },
        tl,
    )
    .await;
    match sol_result {
        CommandResult::Success(detail) => {
            if let Some(expect_path) = judge_path.expect {
                let actual = detail.stdout;
                let expect = read_async(&expect_path).await.unwrap();
                if actual == expect {
                    Verdict::Ac
                } else {
                    Verdict::Wa
                }
            } else {
                Verdict::Ac
            }
        }
        CommandResult::Aborted(_) => Verdict::Re,
        CommandResult::Timeout(_) => Verdict::Tle,
    }
}

fn check_args(args: &TestArgs) -> Result<(), TestArgumentError> {
    let dir = args.directory.clone();
    if !dir.exists() {
        return Err(TestArgumentError::CasedirIsNotFound(dir));
    }
    if !dir.is_dir() {
        return Err(TestArgumentError::CasedirIsNotDirectory(dir));
    }
    Ok(())
}
