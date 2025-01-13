use std::path::PathBuf;

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
    #[error("Judge failed.")]
    JudgeFailed(#[source] crate::judge::batch::Error),
}

pub(super) fn run(args: &Args) -> Result<(), Error> {
    use crate::judge::batch::judge;

    log::info!("[Batch Test] Start");
    let dir = &args.directory;
    if !dir.exists() {
        return Err(Error::CasedirNotFound(dir.to_owned()));
    }
    if !dir.is_dir() {
        return Err(Error::CasedirNotDir(dir.to_owned()));
    }

    let _ = judge(&args.command, dir, args.timelimit_ms.unwrap_or(10000))
        .map_err(Error::JudgeFailed)?;
    log::info!("[Batch Test] End");
    Ok(())
}
