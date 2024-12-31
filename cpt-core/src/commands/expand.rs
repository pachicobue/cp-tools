use std::path::{Path, PathBuf};

use clap::{Args, ValueHint};
use thiserror::Error;

use crate::{
    config::{ensure_expandable, expand_command, guess_lang, ConfigError},
    core::{
        fs::{filename, with_tempdir},
        process::run_command_simple,
    },
    dir, styled,
};

#[derive(Args, Debug)]
pub(crate) struct ExpandArgs {
    /// 入力ファイル
    #[arg(required = true, value_hint = ValueHint::FilePath)]
    file: PathBuf,

    /// 出力先ファイル
    #[arg(short = 'o', long, value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,
}

/// 展開コマンドエラーを表す列挙型
#[derive(Error, Debug)]
pub(crate) enum ExpandCommandError {
    #[error("Invalid argument")]
    InvalidArgument(#[from] ExpandArgumentError),
    #[error("Expand command failed")]
    ExpandCommandError,
}

/// 展開引数エラーを表す列挙型
#[derive(Error, Debug)]
pub(crate) enum ExpandArgumentError {
    /// ソースファイルが見つからないエラー
    #[error("Src file `{0}` is not found.")]
    SourcefileIsNotFound(PathBuf),
    /// ソースパスがファイルでないエラー
    #[error("Src path `{0}` is not a file.")]
    SourcefileIsNotFile(PathBuf),
    /// 言語仕様エラー
    #[error(transparent)]
    ExpandCommandNotFound(#[from] ConfigError),
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
        let expr = expand_command(lang.clone(), &args.file, &dst, tempdir.path());
        let result = run_command_simple(expr);
        if !result.is_success() {
            return Err(ExpandCommandError::ExpandCommandError);
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
    let basedir = dir::workspace_dir();
    basedir.join(filename(filepath) + "_bundled.cpp")
}
