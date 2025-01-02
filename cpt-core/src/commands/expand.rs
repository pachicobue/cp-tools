use std::path::PathBuf;

use clap::{Args, ValueHint};
use thiserror::Error;

use cpt_stdx::path::PathInfo;

use crate::lang::LangError;

#[derive(Args, Debug)]
pub(crate) struct ExpandArgs {
    #[arg(required = true, value_hint = ValueHint::FilePath)]
    file: PathBuf,
    #[arg(short = 'o', long, value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,
}

#[derive(Error, Debug)]
pub(crate) enum ExpandCommandError {
    #[error("Src file `{0}` is not found.")]
    SrcfileNotFound(PathBuf),
    #[error("Src path `{0}` is not a file.")]
    SrcfileNotFile(PathBuf),
    #[error("Language specification error")]
    InvalidLanguage(#[from] LangError),
    #[error("Expand command process failed")]
    ProcessFailed,
}

pub(crate) fn expand(args: &ExpandArgs) -> Result<(), ExpandCommandError> {
    use cpt_stdx::path::PathInfo;
    use cpt_stdx::process::run_command_simple;
    use cpt_stdx::tempfile::with_tempdir;

    use crate::lang::{expand_command, guess_lang};

    log::info!("Expand programs\n{:?}", args);
    let file_pathinfo = PathInfo::new(&args.file);
    if !file_pathinfo.exists {
        return Err(ExpandCommandError::SrcfileNotFound(args.file.to_owned()))?;
    }
    if !file_pathinfo.is_file {
        return Err(ExpandCommandError::SrcfileNotFile(args.file.to_owned()))?;
    }

    let src = args.file.as_ref();
    let dst = args
        .output
        .to_owned()
        .unwrap_or(default_output_path(&file_pathinfo));

    let lang = guess_lang(&file_pathinfo.extension).map_err(ExpandCommandError::InvalidLanguage)?;
    with_tempdir(|tempdir| {
        let expr = expand_command(lang.clone(), src, &dst, tempdir.path())
            .map_err(ExpandCommandError::InvalidLanguage)?;
        let result = run_command_simple(expr);
        if !result.is_success() {
            return Err(ExpandCommandError::ProcessFailed);
        }
        Ok(())
    })?;

    log::info!(
        "Expand completed\nInput : {}\nOutput: {}",
        src.display(),
        dst.display()
    );
    Ok(())
}

fn default_output_path(pathinfo: &PathInfo) -> PathBuf {
    use crate::dir::workspace_dir;
    workspace_dir().join(pathinfo.filestem.to_owned() + "_bundled" + pathinfo.extension.as_str())
}
