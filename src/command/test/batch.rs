use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use clap::{Args, ValueHint};
use color_eyre::eyre::{ensure, Context, OptionExt, Result};
use itertools::Itertools;
use tempfile;

use crate::{
    command::build::{build, BuildArgs},
    config::{
        metadata::crate_name,
        testcase::{collect_testcases, default_casedir},
    },
    fs::{read, read_async},
    judge::{Testcase, Verdict},
    process::{run_multiple, CmdExpression, CmdIoRedirection},
    styled,
    task::{run_multi_task, TaskResult},
};

#[derive(Args, Debug)]
pub(crate) struct BatchArgs {
    /// 入力ファイル(.cppのみ対応)
    #[arg(required(true), value_hint(ValueHint::FilePath))]
    file: PathBuf,

    /// 出力先ファイル
    #[arg(short = 'o', long, value_hint(ValueHint::FilePath))]
    output: Option<PathBuf>,

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

pub(crate) fn batch(args: &BatchArgs) -> Result<()> {
    log::info!("{}\n{:?}", styled!("Batch Test").bold().green(), args);
    check_args(args)?;

    let exe_path = build(&BuildArgs {
        file: args.file.clone(),
        output: args.output.clone(),
        release: args.release.clone(),
    })?;
    let case_dir = match &args.directory {
        Some(dir) => dir.clone(),
        None => default_casedir(&args.file)?,
    };
    let mut testcases = collect_testcases(&case_dir);
    log::debug!("testcases: {:?}", &testcases);
    if testcases.is_empty() {
        log::error!("No testcase found!");
    } else {
        let tempdir = tempfile::Builder::new()
            .prefix(&format!("{}-", crate_name()))
            .tempdir()
            .context("Failed to create tempdir.")?;
        let temppath = tempdir.path();
        for testcase in testcases.iter_mut() {
            testcase.actual = Some(default_actual(temppath, &testcase.input)?);
        }

        let verdicts = generate_actual(&exe_path, &testcases, args.tl)?;
        let verdicts = check(&testcases, &verdicts)?;

        // for (i,verdict) in verdicts.iter().enumerate() {

        // }
        log::info!(
            "{}\nResult: {:?}",
            styled!("Batch Test completed").bold().green(),
            &verdicts
        );
        tempdir.close().context("Failed to close tempdir.")?;
    }
    Ok(())
}

pub(crate) fn generate_actual(
    exe_path: &Path,
    testcases: &Vec<Testcase>,
    tl: Option<f32>,
) -> Result<Vec<Verdict>> {
    let results = run_multiple(
        CmdExpression::new(&exe_path, Vec::<OsString>::new()),
        tl,
        testcases
            .iter()
            .map(|testcase| CmdIoRedirection {
                stdin: testcase.input.clone().into(),
                stdout: testcase.actual.clone().unwrap().into(),
                stderr: None,
            })
            .collect_vec(),
    )?;
    let verdicts = results
        .iter()
        .map(|result| match result {
            TaskResult::Done(_) => Verdict::Wj,
            TaskResult::RuntimeError(_) => Verdict::Re,
            TaskResult::Timeout => Verdict::Tle,
        })
        .collect_vec();
    Ok(verdicts)
}

pub(crate) fn check(testcases: &Vec<Testcase>, verdicts: &Vec<Verdict>) -> Result<Vec<Verdict>> {
    let tasks = testcases
        .into_iter()
        .zip(verdicts.into_iter())
        .map(|(testcase, verdict)| check_single(testcase.to_owned(), verdict.to_owned()))
        .collect_vec();
    let results = run_multi_task(tasks, None)?
        .into_iter()
        .map(|res| match res {
            TaskResult::Done(verdict) => verdict,
            _ => Verdict::Ie,
        })
        .collect_vec();
    Ok(results)
}

async fn check_single(testcase: Testcase, verdict: Verdict) -> Result<TaskResult<Verdict>> {
    if verdict == Verdict::Wj {
        if testcase.expect.is_some() {
            let expect = read_async(&testcase.expect.unwrap()).await?;
            let actual = read_async(&testcase.actual.unwrap()).await?;
            if expect == actual {
                Ok(TaskResult::Done(Verdict::Ac))
            } else {
                Ok(TaskResult::Done(Verdict::Wa))
            }
        } else {
            Ok(TaskResult::Done(Verdict::Ac))
        }
    } else {
        Ok(TaskResult::Done(verdict))
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
