use std::path::{Path, PathBuf};

use clap::{Args, ValueHint};
use thiserror::Error;

use crate::{
    config::{build_command, ensure_buildable, guess_lang, ConfigError},
    core::{
        fs::{filename, with_tempdir},
        process::run_command_simple,
    },
    dir, styled,
};

#[derive(Args, Debug)]
pub(crate) struct BuildArgs {
    #[arg(required = true, value_hint(ValueHint::FilePath))]
    pub(crate) file: PathBuf,
    #[arg(short = 'o', long, value_hint(ValueHint::FilePath))]
    pub(crate) output: Option<PathBuf>,
    #[arg(long, default_value_t = false)]
    pub(crate) release: bool,
}

#[derive(Error, Debug)]
pub(crate) enum BuildCommandError {
    #[error("Invalid argument")]
    InvalidArgument(#[from] ArgumentError),
    #[error("Build command failed")]
    BuildCommandError,
}

#[derive(Error, Debug)]
pub(crate) enum ArgumentError {
    #[error("Src file `{0}` is not found.")]
    SrcfileNotFound(PathBuf),
    #[error("Src path `{0}` is not a file.")]
    SrcfileNotFile(PathBuf),
    #[error("Invalid language")]
    InvalidLanguage(#[from] ConfigError),
}

pub(crate) fn build(args: &BuildArgs) -> Result<(), BuildCommandError> {
    log::info!("{}\n{:?}", styled!("Build program").bold().green(), args);

    check_args(args)?;
    let output = args
        .output
        .to_owned()
        .unwrap_or(default_output_path(&args.file));
    let lang = guess_lang(&args.file).unwrap();
    with_tempdir(|tempdir| {
        let expr = build_command(
            lang.clone(),
            &args.file,
            &output,
            args.release,
            tempdir.path(),
        );
        let result = run_command_simple(expr);
        if !result.is_success() {
            return Err(BuildCommandError::BuildCommandError);
        }
        Ok(())
    })?;

    log::info!(
        "{}\nInput : {}\nOutput: {}",
        styled!("Build completed").bold().green(),
        args.file.display(),
        output.display()
    );
    Ok(())
}

fn check_args(args: &BuildArgs) -> Result<(), ArgumentError> {
    let src_file = args.file.clone();
    if !src_file.exists() {
        return Err(ArgumentError::SrcfileNotFound(src_file));
    }
    if !src_file.is_file() {
        return Err(ArgumentError::SrcfileNotFile(src_file));
    }
    ensure_buildable(&args.file)?;
    Ok(())
}

fn default_output_path(filepath: &Path) -> PathBuf {
    let basedir = dir::workspace_dir();
    basedir.join(filename(filepath))
}
