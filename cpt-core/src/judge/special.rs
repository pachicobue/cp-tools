use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Failed to execute program.")]
    Program(#[source] cpt_stdx::process::Error),
    #[error("Failed to execute judge.")]
    Judge(#[source] cpt_stdx::process::Error),
}

#[derive(Debug, Clone)]
struct JudgeFiles {
    casename: String,
    input: PathBuf,
    actual: PathBuf,
    debug: PathBuf,
    judge: PathBuf,
}
impl JudgeFiles {
    fn new(testcase: crate::testcase::Testcase, dir: &Path) -> Self {
        Self {
            casename: testcase.casename.to_owned(),
            input: testcase.input,

            actual: dir.join(testcase.casename.to_owned() + ".actual.txt"),
            debug: dir.join(testcase.casename.to_owned() + ".debug.txt"),
            judge: dir.join(testcase.casename + ".judge.txt"),
        }
    }
}

/// The special judge program should follow this command-line interface:
/// ```text
/// $ <judge_program> <input_file> <output_file>
/// ```
/// AC:  The judge program ended (Regardless of return code).
/// WA:  The judge program aborted or timeout.
/// RE:  The main program aborted.
/// TLE: The main program timeout.
pub(crate) fn judge(
    program_command: &str,
    judge_command: &str,
    testcase: crate::testcase::Testcase,
    timelimit_ms: u64,
    dir: &Path,
) -> Result<crate::judge::Verdict, Error> {
    use std::process::Stdio;

    use cpt_stdx::fs;
    use cpt_stdx::process::{Command, IoRedirection, Status, StatusSummary};

    use crate::judge::Verdict;

    let judge_files = JudgeFiles::new(testcase, dir);
    log::info!("[Judge][{}] Start", judge_files.casename);
    let Status { summary, detail } = Command::from(program_command)
        .exec(
            IoRedirection {
                stdin: Stdio::from(fs::open(&judge_files.input).unwrap()),
                stdout: Stdio::from(fs::create(&judge_files.actual, true).unwrap()),
                stderr: Stdio::from(fs::create(&judge_files.debug, true).unwrap()),
            },
            timelimit_ms,
            false,
        )
        .map_err(Error::Program)?;
    Ok(match summary {
        StatusSummary::Success => {
            let mut command = Command::from(judge_command);
            command
                .args
                .push(format!("{}", judge_files.input.display()));
            command
                .args
                .push(format!("{}", judge_files.actual.display()));
            let Status { summary, detail: _ } = command
                .exec(
                    IoRedirection {
                        stdin: Stdio::null(),
                        stdout: Stdio::from(fs::create(&judge_files.judge, true).unwrap()),
                        stderr: Stdio::from(fs::create(&judge_files.judge, true).unwrap()),
                    },
                    timelimit_ms * 10,
                    false,
                )
                .map_err(Error::Judge)?;
            match summary {
                StatusSummary::Success => {
                    log::info!("[Judge][{}] AC", judge_files.casename);
                    Verdict::Ac
                }
                _ => {
                    log::warn!("[Judge][{}] WA", judge_files.casename);
                    Verdict::Wa
                }
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
