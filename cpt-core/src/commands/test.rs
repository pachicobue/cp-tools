use std::{path::PathBuf, process::Stdio};

use clap::{Args, ValueHint};
use itertools::Itertools;
use thiserror::Error;

use crate::{
    core::{
        fs::{read_async, with_tempdir},
        judge::{collect_judge_paths, JudgePaths, Verdict},
        process::{command_task, CommandExpression, CommandIoRedirection, CommandResult},
        task::run_tasks,
    },
    styled,
};

/// テストコマンドの引数を格納する構造体
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

#[derive(Error, Debug)]
pub(crate) enum TestCommandError {
    #[error("Invalid argument")]
    InvalidArgument(#[from] ArgumentError),
    #[error("Testcase not found.")]
    CaseNotFound,
}

#[derive(Error, Debug)]
pub(crate) enum ArgumentError {
    #[error("Testcase directory `{0}` is not found.")]
    CasedirNotFound(PathBuf),
    #[error("Testcase path `{0}` is not a directory.")]
    CasedirNotDir(PathBuf),
}

pub(crate) fn test(args: &TestArgs) -> Result<Vec<Verdict>, TestCommandError> {
    log::info!("{}\n{:?}", styled!("Batch Test").bold().green(), args);
    check_args(args)?;

    let verdicts = with_tempdir(|tempdir| -> Result<Vec<Verdict>, TestCommandError> {
        let temppath = tempdir.path();
        let judge_paths = collect_judge_paths(&args.directory, temppath);
        if judge_paths.is_empty() {
            return Err(TestCommandError::CaseNotFound);
        }
        let check_tasks = judge_paths
            .into_iter()
            .map(|judge_path| judge_single(args.command.clone(), judge_path, args.timelimit))
            .collect_vec();
        Ok(run_tasks(check_tasks))
    })?;
    Ok(verdicts)
}

/// 単一のテストケースを判定する関数
///
/// # 引数
///
/// * `command` - 実行コマンド
/// * `judge_path` - 判定に使用するパス
/// * `tl` - タイムリミット
///
/// # 戻り値
///
/// 判定結果
async fn judge_single(command: String, judge_path: JudgePaths, tl: Option<f32>) -> Verdict {
    let sol_result = command_task(
        CommandExpression::new(command, Vec::<String>::new()),
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

fn check_args(args: &TestArgs) -> Result<(), ArgumentError> {
    let dir = args.directory.clone();
    if !dir.exists() {
        return Err(ArgumentError::CasedirNotFound(dir));
    }
    if !dir.is_dir() {
        return Err(ArgumentError::CasedirNotDir(dir));
    }
    Ok(())
}
