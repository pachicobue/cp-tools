use std::path::{Path, PathBuf};
use std::time::Duration;

use clap::{Args, ValueHint};
use itertools::Itertools;
use thiserror::Error;

use cpt_stdx::process::{
    CommandExpression, CommandIoRedirection, CommandResult, CommandResultSummary,
};
use cpt_stdx::task::TaskError;

use crate::judge::{Testcase, Verdict};

#[derive(Args, Debug)]
pub(crate) struct BatchTestArgs {
    #[arg(required = true, short = 'c')]
    command: String,
    #[arg(short = 'd', visible_alias = "dir", value_hint(ValueHint::FilePath))]
    directory: PathBuf,
    #[arg(short = 't', visible_alias = "tl")]
    timelimit: Option<f32>,
}

#[derive(Error, Debug)]
pub(crate) enum BatchTestCommandError {
    #[error("Testcase directory `{0}` is not found.")]
    CasedirNotFound(PathBuf),
    #[error("Testcase path `{0}` is not a directory.")]
    CasedirNotDir(PathBuf),
    #[error("Testcase not found in `{0}`.")]
    CaseNotFound(PathBuf),
    #[error("Test command process failed.")]
    TestProcessFailed(#[from] TaskError),
}

#[derive(Debug, Clone)]
struct JudgeResult {
    verdict: Verdict,
    elapsed: Duration,
}

#[derive(Debug, Clone)]
struct JudgeFiles {
    casename: String,
    input: PathBuf,
    expect: Option<PathBuf>,
    actual: PathBuf,
}
impl JudgeFiles {
    fn new(testcase: Testcase, tempdir: impl AsRef<Path>) -> Self {
        Self {
            casename: testcase.casename.to_owned(),
            input: testcase.input,
            expect: testcase.output,
            actual: tempdir.as_ref().join(testcase.casename + "_actual.txt"),
        }
    }
}

pub(crate) fn test(args: &BatchTestArgs) -> Result<Vec<Verdict>, BatchTestCommandError> {
    use cpt_stdx::task::run_tasks;
    use cpt_stdx::tempfile::with_tempdir;

    use crate::judge::collect_cases;

    log::info!("Batch Test\n{:?}", args);

    let dir = args.directory.to_owned();
    if !dir.exists() {
        return Err(BatchTestCommandError::CasedirNotFound(dir));
    }
    if !dir.is_dir() {
        return Err(BatchTestCommandError::CasedirNotDir(dir));
    }

    with_tempdir(|tempdir| -> Result<Vec<Verdict>, BatchTestCommandError> {
        let tempdir = tempdir.path();
        let testcases = collect_cases(&args.directory);
        if testcases.is_empty() {
            return Err(BatchTestCommandError::CaseNotFound(
                args.directory.to_owned(),
            ));
        }

        let check_tasks = testcases
            .into_iter()
            .map(|case| JudgeFiles::new(case, tempdir))
            .map(|judge_files| judge_single(args.command.to_owned(), judge_files, args.timelimit))
            .collect_vec();
        run_tasks(check_tasks).map_err(BatchTestCommandError::TestProcessFailed)
    })
}

async fn judge_single(command: String, judge_files: JudgeFiles, tl: Option<f32>) -> Verdict {
    use cpt_stdx::fs;
    use cpt_stdx::process::command_task;

    let CommandResult { summary, detail } = command_task(
        CommandExpression::from(&command),
        CommandIoRedirection::piped(),
        tl,
    )
    .await;
    match summary {
        CommandResultSummary::Success => {
            if let Some(expect_path) = judge_files.expect {
                let actual = detail.stdout;
                fs::write(judge_files.actual, actual.to_owned(), false);
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
