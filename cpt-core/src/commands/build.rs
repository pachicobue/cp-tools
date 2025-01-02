use std::path::PathBuf;

use clap::{Args, ValueHint};
use thiserror::Error;

use cpt_stdx::path::PathInfo;

use crate::lang::LangError;

#[derive(Args, Debug)]
pub(crate) struct BuildArgs {
    #[arg(required = true, value_hint(ValueHint::FilePath))]
    pub file: PathBuf,
    #[arg(short = 'o', long, value_hint(ValueHint::FilePath))]
    pub output: Option<PathBuf>,
    #[arg(long, default_value_t = false)]
    pub release: bool,
}

#[derive(Error, Debug)]
pub(crate) enum BuildCommandError {
    #[error("Src file `{0}` is not found.")]
    SrcfileNotFound(PathBuf),
    #[error("Src path `{0}` is not a file.")]
    SrcfileNotFile(PathBuf),
    #[error("Language specification error")]
    InvalidLanguage(#[from] LangError),
    #[error("Build command process failed")]
    ProcessFailed,
}

pub(crate) fn build(args: &BuildArgs) -> Result<(), BuildCommandError> {
    use cpt_stdx::process::run_command_simple;
    use cpt_stdx::tempfile::with_tempdir;

    use crate::lang::{build_command, guess_lang};

    log::info!("Build program\n{:?}", args);
    let file_pathinfo = PathInfo::new(&args.file);
    if !file_pathinfo.exists {
        return Err(BuildCommandError::SrcfileNotFound(args.file.to_owned()))?;
    }
    if !file_pathinfo.is_file {
        return Err(BuildCommandError::SrcfileNotFile(args.file.to_owned()))?;
    }

    let src = args.file.as_ref();
    let dst = args
        .output
        .to_owned()
        .unwrap_or(default_output_path(&file_pathinfo));

    let lang = guess_lang(&file_pathinfo.extension).map_err(BuildCommandError::InvalidLanguage)?;
    with_tempdir(|tempdir| {
        let expr = build_command(lang.clone(), src, &dst, args.release, tempdir.path())
            .map_err(BuildCommandError::InvalidLanguage)?;
        let result = run_command_simple(expr);
        if !result.is_success() {
            return Err(BuildCommandError::ProcessFailed);
        }
        Ok(())
    })?;

    log::info!(
        "Build completed\nInput : {}\nOutput: {}",
        src.display(),
        dst.display()
    );
    Ok(())
}

fn default_output_path(pathinfo: &PathInfo) -> PathBuf {
    use crate::dir::workspace_dir;
    workspace_dir().join(pathinfo.filestem.to_owned() + "_exe")
}
