use std::path::PathBuf;

#[derive(clap::Args, Debug)]
pub(crate) struct Args {
    #[arg(required = true, short = 'c')]
    command: String,
    #[arg(required = true, short = 'j')]
    judge: String,
    #[arg(required = true, short = 'i')]
    input_generator: String,
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
    #[error("Generation failed.")]
    GenerationFailed(#[source] crate::generator::Error),
    #[error("Judge failed.")]
    JudgeFailed(#[source] crate::judge::reactive::Error),
    #[error("Failed to copy testcase.")]
    TestcaseCopy(#[from] crate::testcase::Error),
}

pub(super) fn run(args: &Args) -> Result<(), Error> {
    use crate::generator::generate;
    use crate::judge::reactive::judge;

    log::info!("[Reactive Hack] Start");
    let dir = &args.directory;
    if !dir.exists() {
        return Err(Error::CasedirNotFound(dir.to_owned()));
    }
    if !dir.is_dir() {
        return Err(Error::CasedirNotDir(dir.to_owned()));
    }

    let timelimit_program = args.timelimit_ms.unwrap_or(10000);
    let timelimit_generator = timelimit_program * 10;
    let mut trial = 0;
    let (temp_case, final_case) = crate::testcase::new_hackcase(dir);
    loop {
        trial += 1;
        log::info!("[Batch Hack][Trial {}] Start", trial);
        let case = generate(&temp_case, &args.input_generator, &None, timelimit_generator)
            .map_err(Error::GenerationFailed)?;

        let verdict = judge(&args.command, &args.judge, case, timelimit_program, dir)
            .map_err(Error::JudgeFailed)?;
        log::info!("[Batch Hack][Trial {}] End: {}", trial, verdict);
        if !verdict.is_ac() {
            temp_case.copy_to(&final_case)?;
            break;
        }
    }

    log::info!("[Reactive Hack] End",);
    Ok(())
}
