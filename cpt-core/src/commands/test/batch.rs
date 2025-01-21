use std::path::PathBuf;

use itertools::Itertools;

#[derive(clap::Args, Debug)]
pub(crate) struct Args {
    #[arg(required = true, short = 'c')]
    command: String,
    #[arg(required = true, short = 'd', value_hint(clap::ValueHint::FilePath))]
    directory: PathBuf,
    #[arg(required = false, short = 't')]
    timelimit_ms: Option<u64>,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("`{0}` is not found.")]
    CasedirNotFound(PathBuf),
    #[error("`{0}` is not a directory.")]
    CasedirNotDir(PathBuf),
    #[error("No case found in `{0}`.")]
    CaseNotFound(PathBuf),
    #[error("Judge failed.")]
    JudgeFailed(#[from] crate::judge::batch::Error),
}

pub(super) fn run(args: &Args) -> Result<(), Error> {
    use strum::{EnumCount, IntoEnumIterator};

    use crate::judge::batch::judge;
    use crate::judge::Verdict;

    log::info!("[Batch Test] Start");
    let dir = &args.directory;
    if !dir.exists() {
        return Err(Error::CasedirNotFound(dir.to_owned()));
    }
    if !dir.is_dir() {
        return Err(Error::CasedirNotDir(dir.to_owned()));
    }

    let cases = crate::testcase::collect(dir);
    if cases.is_empty() {
        return Err(Error::CaseNotFound(dir.to_owned()));
    }

    let timelimit = args.timelimit_ms.unwrap_or(10000);
    let mut results = [0; Verdict::COUNT];
    for case in cases {
        let verdict = judge(&args.command, case, timelimit, dir)?;
        results[verdict as usize] += 1;
    }
    log::info!(
        "[Batch Test] End ({})",
        Verdict::iter()
            .map(|verdict| format!("{}:{}", verdict.to_owned(), results[verdict as usize]))
            .join(",")
    );
    Ok(())
}
