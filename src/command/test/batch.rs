use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Stdio,
};

use clap::{Args, ValueHint};
use color_eyre::eyre::{ensure, Context, OptionExt, Result};
use itertools::Itertools;
use tempfile;

use crate::{
    command::build::{build, BuildArgs},
    config::metadata::crate_name,
    fs::read_async,
    judge::{collect_judge_paths, default_casedir, JudgePaths, Verdict},
    process::{command_task, CommandExpression, CommandIoRedirection, CommandResult},
    styled,
    task::run_tasks,
};

#[derive(Args, Debug)]
pub(crate) struct BatchArgs {
    /// 入力ファイル(.cppのみ対応)
    #[arg(required(true), value_hint(ValueHint::FilePath))]
    file: PathBuf,

    /// テストディレクトリ
    #[arg(short = 'd', alias = "dir", value_hint(ValueHint::FilePath))]
    directory: Option<PathBuf>,

    /// コンパイル最適化フラグ
    #[arg(long, default_value_t = false)]
    release: bool,

    /// TL(秒単位)
    #[arg(long)]
    tl: Option<f32>,
}

pub(crate) fn batch(args: &BatchArgs) -> Result<Vec<Verdict>> {
    log::info!("{}\n{:?}", styled!("Batch Test").bold().green(), args);
    check_args(args)?;

    let exe_path = build(&BuildArgs {
        file: args.file.clone(),
        output: None,
        release: args.release.clone(),
    })?;
    let case_dir = match &args.directory {
        Some(dir) => dir.clone(),
        None => default_casedir(&args.file)?,
    };
    let tempdir = tempfile::Builder::new()
        .prefix(&format!("{}-", crate_name()))
        .tempdir()
        .wrap_err("Failed to create tempdir.")?;
    let temppath = tempdir.path();

    let judge_paths = collect_judge_paths(&case_dir, temppath);
    log::debug!("testcases: {:?}", &judge_paths);
    if judge_paths.is_empty() {
        log::error!("No testcase found!");
    }
    let check_tasks = judge_paths
        .into_iter()
        .map(|judge_path| judge_single(exe_path.clone(), judge_path, args.tl))
        .collect_vec();
    let verdicts = run_tasks(check_tasks)?;
    tempdir.close().wrap_err("Failed to close tempdir.")?;
    Ok(verdicts)
}

async fn judge_single(
    exe_path: PathBuf,
    judge_path: JudgePaths,
    tl: Option<f32>,
) -> Result<Verdict> {
    let sol_result = command_task(
        CommandExpression::new(exe_path, &Vec::<OsString>::new()),
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
        args.file.exists(),
        "Input File {} not found.",
        args.file.to_string_lossy()
    );
    ensure!(
        args.file.extension().ok_or_eyre("Failed to get ext.")? == "cpp",
        "Only .cpp file is supported."
    );
    if let Some(test_dir) = &args.directory {
        ensure!(
            test_dir.exists(),
            "Testdir {} not found.",
            test_dir.to_string_lossy()
        );
        ensure!(
            test_dir.is_dir(),
            "{} is not a directory.",
            test_dir.to_string_lossy()
        );
    }
    Ok(())
}

fn default_actual(basedir: &Path, input_path: &Path) -> Result<PathBuf> {
    let filename = input_path
        .file_stem()
        .ok_or_eyre("Failed to get file stem.")?;
    Ok(basedir.join(&format!("{}.actual", filename.to_string_lossy())))
}
