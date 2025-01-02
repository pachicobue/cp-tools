use std::path::PathBuf;

use clap::{Args, ValueHint};
use itertools::Itertools;
use thiserror::Error;

use cpt_stdx::process::{
    CommandExpression, CommandIoRedirection, CommandResult, CommandResultSummary,
};
use cpt_stdx::task::TaskError;

use crate::judge::{JudgePaths, Verdict};

#[derive(Args, Debug)]
pub(crate) struct TestArgs {
    #[arg(required = true, short = 'c')]
    command: String,
    #[arg(short = 'd', visible_alias = "dir", value_hint(ValueHint::FilePath))]
    directory: PathBuf,
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
    #[error("Test command process failed.")]
    TestProcessFailed(#[from] TaskError),
}

pub(crate) fn test(args: &TestArgs) -> Result<Vec<Verdict>, TestCommandError> {
    use cpt_stdx::task::run_tasks;
    use cpt_stdx::tempfile::with_tempdir;

    use crate::judge::collect_judge_paths;

    log::info!("Batch Test\n{:?}", args);

    let dir = args.directory.to_owned();
    if !dir.exists() {
        return Err(TestCommandError::CasedirNotFound(dir));
    }
    if !dir.is_dir() {
        return Err(TestCommandError::CasedirNotDir(dir));
    }

    with_tempdir(|tempdir| -> Result<Vec<Verdict>, TestCommandError> {
        let temppath = tempdir.path();
        let judge_paths = collect_judge_paths(&args.directory, temppath);
        if judge_paths.is_empty() {
            return Err(TestCommandError::CaseNotFound(args.directory.to_owned()));
        }
        let check_tasks = judge_paths
            .into_iter()
            .map(|judge_path| judge_single(args.command.clone(), judge_path, args.timelimit))
            .collect_vec();
        run_tasks(check_tasks).map_err(TestCommandError::TestProcessFailed)
    })
}

async fn judge_single(command: String, judge_path: JudgePaths, tl: Option<f32>) -> Verdict {
    use cpt_stdx::fs;
    use cpt_stdx::process::command_task;

    let CommandResult { summary, detail } = command_task(
        CommandExpression::new(command, Vec::<String>::new()),
        CommandIoRedirection::default(),
        tl,
    )
    .await;
    match summary {
        CommandResultSummary::Success => {
            if let Some(expect_path) = judge_path.expect {
                let actual = detail.stdout;
                if let Ok(expect) = fs::read(&expect_path) {
                    if actual == expect {
                        Verdict::Ac
                    } else {
                        Verdict::Wa
                    }
                } else {
                    Verdict::Ie
                }
            } else {
                Verdict::Ac
            }
        }
        CommandResultSummary::Aborted => Verdict::Re,
        CommandResultSummary::Timeout => Verdict::Tle,
    }
}
