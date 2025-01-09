mod expand;
mod isystem;

use std::path::{Path, PathBuf};

use clap::{ArgAction, Parser, ValueHint};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use cpt_stdx::fs::{self, FilesystemError};
use thiserror::Error;

use cpt_stdx::path::PathInfo;
use cpt_stdx::trace_error::error_trace;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    #[arg(required = true, value_hint(ValueHint::FilePath))]
    file: PathBuf,
    #[arg(required = false, short = 'o', long = "output")]
    output: Option<PathBuf>,
    #[arg(
        short = 'I',
        long = "include_directories",
        value_hint(ValueHint::FilePath)
    )]
    include_directories: Vec<PathBuf>,
    #[arg(short = 'D', long = "defined_macros")]
    defined_macros: Vec<String>,
    #[arg(long = "std")]
    std: Option<String>,
    #[arg(long = "with_comment", action=ArgAction::SetTrue)]
    with_comment: bool,
}

#[derive(Debug, Error)]
enum Error {
    #[error("File `{0}` not found")]
    FileNotFound(PathBuf),
    #[error("File `{0}` is not file")]
    FileNotFile(PathBuf),
    #[error("Both `clang++` and `g++` not found")]
    CompilerNotFound,
    #[error("Failed to get system header dependency:\n{0}")]
    FailedToGetDependency(isystem::Error),
    #[error("Failed to expand:\n{0}")]
    FailedToExpand(expand::Error),
    #[error("Failed to output:\n{0}")]
    FailedToOutput(FilesystemError),
}

fn main() {
    let res = main_inner();
    if res.is_err() {
        log::error!("{:#?}", res);
        let msg = error_trace(&res.unwrap_err());
        log::error!("{}", msg);
    }
}

fn main_inner() -> Result<(), Error> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    log::info!("cpp_bundle {:#?}", args);
    let src = args.file;
    let dst = args.output.unwrap_or(default_output(&src));
    if !src.exists() {
        return Err(Error::FileNotFound(src.to_owned()));
    }
    if !src.is_file() {
        return Err(Error::FileNotFile(src.to_owned()));
    }

    let cxx = find_cxx()?;
    let system_deps = isystem::get_system_header_deps(
        &cxx,
        &src,
        &args.include_directories,
        &args.defined_macros,
        &args.std,
    )
    .map_err(Error::FailedToGetDependency)?;
    log::trace!("System headers: \n {:#?}", system_deps);
    let expanded = expand::expand(
        &cxx,
        &src,
        &args.include_directories,
        &args.defined_macros,
        &args.std,
        args.with_comment,
        system_deps,
    )
    .map_err(Error::FailedToExpand)?;
    log::trace!("{}", expanded);

    fs::write(dst.to_owned(), expanded, true).map_err(Error::FailedToOutput)?;
    log::info!("Expanded -> `{}`", dst.display());

    Ok(())
}

fn find_cxx() -> Result<String, Error> {
    ["clang++".to_owned(), "g++".to_owned()]
        .into_iter()
        .find(|cxx| which::which(cxx).is_ok())
        .ok_or(Error::CompilerNotFound)
}

fn default_output(file: &Path) -> PathBuf {
    let pathinfo = PathInfo::from(file);
    PathInfo::new(
        pathinfo.basedir,
        pathinfo.filestem + "_bundle",
        pathinfo.extension,
    )
    .path
}
