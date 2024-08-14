use std::path::{Path, PathBuf};

use clap::{Args, ValueHint};
use color_eyre::eyre::{ensure, OptionExt, Result};

use crate::{
    compilation::{CompileCommand, CompileMode},
    styled,
};

#[derive(Args, Debug)]
pub(crate) struct BuildArgs {
    /// 入力ファイル(.cppのみ対応)
    #[arg(required = true, value_hint(ValueHint::FilePath))]
    pub(crate) file: PathBuf,

    /// 出力先ファイル
    #[arg(short = 'o', long, value_hint(ValueHint::FilePath))]
    pub(crate) output: Option<PathBuf>,

    /// コンパイル最適化フラグ
    #[arg(long, default_value_t = false)]
    pub(crate) release: bool,
}

/// ファイルのビルドを行う
///
///
/// ## Args
/// - `args`: 展開処理の引数
///   - `file`: 入力ファイル(.cppのみ対応)
///   - `output`: 出力先ファイル(未指定時は`${basename}.exe`)
///   - `release`: コンパイル最適化フラグ(デフォルトはfalse)
///
/// ## Note
/// - ファイルはcppファイルのみ対応
/// - ビルド結果は`output`で指定されたファイルに出力される
///  - `output`が指定されない場合は、`${basename}.exe`に出力される
///  - `${basename}`は`file`の拡張子を除いたもの
/// - コンパイル最適化フラグが指定された場合、リリースモードでビルドされる
pub(crate) fn build(args: &BuildArgs) -> Result<PathBuf> {
    log::info!("{}\n{:?}", styled!("Build program").bold().green(), args);

    check_args(args)?;

    let command = CompileCommand::load_config(if args.release {
        CompileMode::Release
    } else {
        CompileMode::Debug
    })?;
    let dst = match &args.output {
        Some(s) => PathBuf::from(s),
        None => default_exe(&args.file),
    };
    command.exec_compilation(&args.file, Some(&dst))?;

    log::info!(
        "{}\nInput : {}\nOutput: {}",
        styled!("Build completed").bold().green(),
        args.file.display(),
        dst.display()
    );
    Ok(dst)
}

pub(crate) fn default_exe(cpp_path: &Path) -> PathBuf {
    PathBuf::from(cpp_path).with_extension("exe")
}

fn check_args(args: &BuildArgs) -> Result<()> {
    ensure!(
        args.file.exists(),
        "Input File {} not found.",
        args.file.to_string_lossy()
    );
    ensure!(
        args.file.extension().ok_or_eyre("Failed to get ext.")? == "cpp",
        "Only .cpp file is supported."
    );
    Ok(())
}
