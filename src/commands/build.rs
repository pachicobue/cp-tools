use std::path::PathBuf;

use clap::{Args, ValueHint};

use crate::{
    core::{
        error::{BuildArgumentError, BuildCommandError},
        fs::{create_sync, with_tempdir},
        language::{build_command, ensure_buildable, guess_lang},
        process::run_command_simple,
    },
    styled,
};

/// ビルドコマンドの引数を格納する構造体
#[derive(Args, Debug)]
pub(crate) struct BuildArgs {
    /// 入力ファイル
    #[arg(required = true, value_hint(ValueHint::FilePath))]
    pub(crate) file: PathBuf,

    /// 出力先ファイル
    #[arg(short = 'o', long, value_hint(ValueHint::FilePath))]
    pub(crate) output: PathBuf,

    /// コンパイル最適化フラグ
    #[arg(long, default_value_t = false)]
    pub(crate) release: bool,
}

/// ビルドコマンドを実行する関数
///
/// # 引数
///
/// * `args` - ビルドコマンドの引数
///
/// # 戻り値
///
/// ビルドに成功した場合は`Ok(())`、失敗した場合は`BuildCommandError`を返す
pub(crate) fn build(args: &BuildArgs) -> Result<(), BuildCommandError> {
    log::info!("{}\n{:?}", styled!("Build program").bold().green(), args);

    check_args(args)?;
    let lang = guess_lang(&args.file).unwrap();
    with_tempdir(|tempdir| {
        let exprs = build_command(
            lang.clone(),
            &args.file,
            &args.output,
            args.release,
            tempdir.path(),
        );
        for expr in exprs {
            let result = run_command_simple(expr);
            if !result.is_success() {
                return Err(BuildCommandError::BuildCommandError);
            }
        }
        Ok(())
    })?;

    log::info!(
        "{}\nInput : {}\nOutput: {}",
        styled!("Build completed").bold().green(),
        args.file.display(),
        args.output.display()
    );
    Ok(())
}

/// ビルドコマンドの引数をチェックする関数
///
/// # 引数
///
/// * `args` - ビルドコマンドの引数
///
/// # 戻り値
///
/// 引数が有効な場合は`Ok(())`、無効な場合は`BuildArgumentError`を返す
fn check_args(args: &BuildArgs) -> Result<(), BuildArgumentError> {
    let src_file = args.file.clone();
    if !src_file.exists() {
        return Err(BuildArgumentError::SourcefileIsNotFound(src_file));
    }
    if !src_file.is_file() {
        return Err(BuildArgumentError::SourcefileIsNotFile(src_file));
    }
    ensure_buildable(&args.file)?;
    Ok(())
}
