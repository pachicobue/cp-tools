use std::path::{Path, PathBuf};

use itertools::Itertools;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Failed to execute program.")]
    CommandFailed(#[source] cpt_stdx::process::Error),
    #[error("Cannot read `expect` file.")]
    ReadExpectFailed(#[source] cpt_stdx::fs::Error),
}

#[derive(Debug, Clone)]
struct JudgeFiles {
    casename: String,
    input: PathBuf,
    expect: Option<PathBuf>,
    actual: PathBuf,
    debug: PathBuf,
    judge: PathBuf,
}
impl JudgeFiles {
    fn new(testcase: crate::judge::Testcase, dir: &Path) -> Self {
        Self {
            casename: testcase.casename.to_owned(),
            input: testcase.input,
            expect: testcase.output,

            actual: dir.join(testcase.casename.to_owned() + ".actual.txt"),
            debug: dir.join(testcase.casename.to_owned() + ".debug.txt"),
            judge: dir.join(testcase.casename + ".judge.txt"),
        }
    }
}

pub(crate) fn judge(
    command: &str,
    dir: &Path,
    timelimit_ms: u64,
) -> Result<crate::judge::Verdict, Error> {
    use strum::{EnumCount, IntoEnumIterator};

    use crate::judge::{collect_cases, Verdict};

    log::info!("[Judge] Start (mode: BatchTest)");
    let mut results = [0; Verdict::COUNT];
    for case in collect_cases(dir) {
        match judge_single(command, JudgeFiles::new(case, dir), timelimit_ms) {
            Err(e) => {
                log::error!("[Judge] Judge program failed!");
                return Err(e);
            }
            Ok(verdict) => {
                results[verdict as usize] += 1;
            }
        };
    }
    log::info!(
        "[Judge] End ({})",
        Verdict::iter()
            .map(|verdict| format!("{}:{}", verdict.to_owned(), results[verdict as usize]))
            .join(",")
    );
    Ok(Verdict::Ac)
}

fn judge_single(
    command: &str,
    judge_files: JudgeFiles,
    timelimit_ms: u64,
) -> Result<crate::judge::Verdict, Error> {
    use std::process::Stdio;

    use similar::{ChangeTag, TextDiff};

    use cpt_stdx::fs;
    use cpt_stdx::process::{Command, IoRedirection, Status, StatusSummary};

    use crate::judge::Verdict;

    log::info!("[Judge][{}] Start", judge_files.casename);
    let Status { summary, detail } = Command::from(command)
        .exec(
            IoRedirection {
                stdin: Stdio::from(fs::open(judge_files.input).unwrap()),
                stdout: Stdio::from(fs::create(&judge_files.actual, true).unwrap()),
                stderr: Stdio::from(fs::create(&judge_files.debug, true).unwrap()),
            },
            timelimit_ms,
            false,
        )
        .map_err(Error::CommandFailed)?;
    Ok(match summary {
        StatusSummary::Success => {
            if let Some(expect_path) = judge_files.expect {
                let actual = fs::read(&judge_files.actual).unwrap();
                let expect = fs::read(&expect_path).map_err(Error::ReadExpectFailed)?;
                if actual == expect {
                    log::info!("[Judge][{}] AC", judge_files.casename);
                    Verdict::Ac
                } else {
                    let diff_lines = TextDiff::from_lines(&expect, &actual)
                        .iter_all_changes()
                        .map(|change| {
                            let sign = match change.tag() {
                                ChangeTag::Delete => "-",
                                ChangeTag::Insert => "+",
                                ChangeTag::Equal => " ",
                            };
                            format!("{} {}", sign, change.to_string().trim_end())
                        })
                        .collect_vec();
                    log::warn!("[Judge][{}] WA", judge_files.casename);
                    diff_lines
                        .to_owned()
                        .iter()
                        .for_each(|line| log::warn!("{}", line));
                    fs::write(judge_files.judge, diff_lines.into_iter().join("\n"), true).unwrap();
                    Verdict::Wa
                }
            } else {
                log::info!("[Judge][{}] AC", judge_files.casename);
                Verdict::Ac
            }
        }
        StatusSummary::Aborted => {
            log::warn!("[Judge][{}] RE", judge_files.casename);
            log::warn!("{}", fs::read(&judge_files.debug).unwrap());
            Verdict::Re
        }
        StatusSummary::Timeout => {
            log::warn!(
                "[Judge][{}] TLE ({}ms/{}ms)",
                judge_files.casename,
                detail.elapsed_ms,
                timelimit_ms
            );
            Verdict::Tle
        }
    })
}
