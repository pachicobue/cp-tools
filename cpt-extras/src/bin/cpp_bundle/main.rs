mod isystem;
mod preprocess;

use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    #[arg(required = true, value_hint(clap::ValueHint::FilePath))]
    src: PathBuf,
    #[arg(required = true, short = 'o', long = "output")]
    dst: PathBuf,
    #[arg(required = false, short = 'c', long = "with_comment", action = clap::ArgAction::SetTrue)]
    with_comment: bool,
    #[arg(required = false, trailing_var_arg = true)]
    cxx_args: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("File `{0}` is not found.")]
    FileNotFound(PathBuf),
    #[error("File `{0}` is not file.")]
    FileNotFile(PathBuf),
    #[error("Both `clang++` and `g++` are not found.")]
    CompilerNotFound,
    #[error("Could not get isystem dependencies.")]
    GetDependenciesFailed(#[source] isystem::Error),
    #[error("Preprocess failed.")]
    PreprocessFailed(#[source] preprocess::Error),
    #[error("Cannot output file.")]
    OutputFailed(#[source] cpt_stdx::fs::Error),
}

fn main() {
    use cpt_stdx::error::stacktrace;
    if let Err(e) = main_inner() {
        log::error!("Error occurred\n{}", stacktrace(e));
    }
}

fn main_inner() -> Result<(), Error> {
    let Cli {
        verbose,
        src,
        dst,
        with_comment,
        cxx_args,
    } = Cli::parse();
    env_logger::Builder::new()
        .filter_level(verbose.log_level_filter())
        .init();
    if !src.exists() {
        return Err(Error::FileNotFound(src.to_owned()));
    }
    if !src.is_file() {
        return Err(Error::FileNotFile(src.to_owned()));
    }

    log::info!("[cpp_bundle] Start");
    let cxx = find_cxx()?;
    let system_deps =
        isystem::get_isys_deps(&cxx, &src, &cxx_args).map_err(Error::GetDependenciesFailed)?;
    log::trace!("System headers: \n {:#?}", system_deps);
    let expanded = preprocess::expand(&cxx, &src, &cxx_args, with_comment, system_deps)
        .map_err(Error::PreprocessFailed)?;
    log::trace!("{}", expanded);

    cpt_stdx::fs::write(&dst, expanded, true).map_err(Error::OutputFailed)?;
    log::info!("[cpp_bundle] End");
    Ok(())
}

fn find_cxx() -> Result<String, Error> {
    ["clang++".to_owned(), "g++".to_owned()]
        .into_iter()
        .find(|cxx| which::which(cxx).is_ok())
        .ok_or(Error::CompilerNotFound)
}
