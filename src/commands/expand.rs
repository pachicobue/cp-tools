use std::path::{Path, PathBuf};

use clap::{Args, ValueHint};

use crate::{
    config::dirs::project_workdir,
    core::{
        error::{ExpandArgumentError, ExpandCommandError},
        fs::{filename, with_tempdir, write_sync},
        language::{ensure_expandable, expand_command, guess_lang},
        process::run_command_simple,
    },
    styled,
};

#[derive(Args, Debug)]
pub(crate) struct ExpandArgs {
    /// 入力ファイル(.cppのみ対応)
    #[arg(required = true, value_hint = ValueHint::FilePath)]
    file: PathBuf,

    /// 出力先ファイル
    #[arg(short = 'o', long, value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,
}

/// #includeの展開処理を行う
///
/// ## Args
/// - `args`: 展開処理の引数
///   - `file`: 入力ファイル(.cppのみ対応)
///   - `output`: 出力先ファイル
///
/// ## Note
/// - 展開処理の結果は`output`で指定されたファイルに出力される
///   - `output`が指定されない場合は、`Bundled.cpp`に出力される
pub(crate) fn expand(args: &ExpandArgs) -> Result<(), ExpandCommandError> {
    log::info!("{}\n{:?}", styled!("Expand program").bold().green(), args);
    check_args(args)?;

    let src = args.file.clone();
    let dst = match &args.output {
        Some(s) => PathBuf::from(&s),
        None => default_output_path(&args.file),
    };

    let lang = guess_lang(&src).unwrap();
    with_tempdir(|tempdir| {
        let exprs = expand_command(lang.clone(), &args.file, &dst, tempdir.path());
        for expr in exprs {
            let result = run_command_simple(expr);
            if !result.is_success() {
                return Err(ExpandCommandError::ExpandCommandError);
            }
        }
        Ok(())
    })?;

    log::info!(
        "{}\nInput : {}\nOutput: {}",
        styled!("Expand completed").bold().green(),
        args.file.display(),
        dst.display()
    );
    Ok(())
}

fn check_args(args: &ExpandArgs) -> Result<(), ExpandArgumentError> {
    let src_file = args.file.clone();
    if !src_file.exists() {
        return Err(ExpandArgumentError::SourcefileIsNotFound(src_file));
    }
    if !src_file.is_file() {
        return Err(ExpandArgumentError::SourcefileIsNotFile(src_file));
    }
    ensure_expandable(&src_file)?;
    Ok(())
}

fn default_output_path(filepath: &Path) -> PathBuf {
    let basedir = project_workdir(filepath.parent().unwrap());
    basedir.join(filename(filepath) + "_bundled.cpp")
}
