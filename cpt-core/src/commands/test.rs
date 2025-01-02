use std::path::PathBuf;
use std::process::Stdio;

use clap::{Args, ValueHint};
use itertools::Itertools;
use thiserror::Error;

use crate::judge::{collect_judge_paths, JudgePaths, Verdict};

#[derive(Args, Debug)]
pub(crate) struct TestArgs {
    #[arg(required = true, value_hint(ValueHint::FilePath))]
    file: PathBuf,
    #[arg(short = 'o', long, value_hint(ValueHint::FilePath))]
    output: Option<PathBuf>,
    #[arg(long, default_value_t = false)]
    release: bool,
    #[arg(short = 'd', visible_alias = "dir", value_hint(ValueHint::FilePath))]
    directory: Option<PathBuf>,
    #[arg(short = 't', visible_alias = "tl")]
    timelimit: Option<f32>,
}

#[derive(Error, Debug)]
pub(crate) enum TestCommandError {
    #[error("Testcase directory `{0}` is not found.")]
    CasedirNotFound(PathBuf),
    #[error("Testcase path `{0}` is not a directory.")]
    CasedirNotDir(PathBuf),
    #[error("Testcase not found in `{0}`.")]
    CaseNotFound(PathBuf),
}

#[derive(Error, Debug)]
pub(crate) enum ArgumentError {}

pub(crate) fn test(args: &TestArgs) -> Result<Vec<Verdict>, TestCommandError> {
    use cpt_stdx::path::PathInfo;
    use cpt_stdx::process::run_command_simple;
    use cpt_stdx::tempfile::with_tempdir;

    log::info!("Batch Test\n{:?}", args);
    check_args(args)?;

    let verdicts = with_tempdir(|tempdir| -> Result<Vec<Verdict>, TestCommandError> {
        let temppath = tempdir.path();
        let judge_paths = collect_judge_paths(&args.directory, temppath);
        if judge_paths.is_empty() {
            return Err(TestCommandError::CaseNotFound(args.directory.to_owned()));
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
    let CommandResult { summary, detail } = command_task(
        CommandExpression::new(command, Vec::<String>::new()),
        CommandIoRedirection {
            stdin: Stdio::piped(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        },
        tl,
    )
    .await;
    match summary {
        CommandResultSummary::Success => {
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
        CommandResultSummary::Aborted => Verdict::Re,
        CommandResultSummary::Timeout => Verdict::Tle,
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
