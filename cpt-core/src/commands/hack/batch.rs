use std::path::PathBuf;

#[derive(clap::Args, Debug)]
pub(crate) struct Args {
    #[arg(required = true, short = 'c')]
    command: String,
    #[arg(required = true, short = 'i')]
    input_generator: String,
    #[arg(required = false, short = 'o')]
    output_generator: Option<String>,
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
    GenerationFailed(#[from] crate::generator::Error),
    #[error("Judge failed.")]
    JudgeFailed(#[from] crate::judge::batch::Error),
    #[error("Failed to copy testcase.")]
    TestcaseCopy(#[from] crate::testcase::Error),
}

pub(super) fn run(args: &Args) -> Result<(), Error> {
    use crate::generator::generate;
    use crate::judge::batch::judge;

    log::info!("[Batch Hack] Start");
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
        let case = generate(
            &temp_case,
            &args.input_generator,
            &args.output_generator,
            timelimit_generator,
        )?;
        let temp_dir = std::env::temp_dir();
        let verdict = judge(&args.command, case, timelimit_program, &temp_dir)?;
        log::info!("[Batch Hack][Trial {}] End: {}", trial, verdict);
        if !verdict.is_ac() {
            temp_case.copy_to_with_intermediate_files(&final_case, &temp_dir, dir)?;
            break;
        }
    }

    log::info!("[Batch Hack] End",);
    Ok(())
}
