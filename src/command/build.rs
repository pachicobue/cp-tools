use crate::utility::compile::{CompileCommand, CompileMode};
use std::path::PathBuf;
use ::{
    clap::{Args, ValueHint},
    color_eyre::eyre::{ensure, Result},
    console::style,
};

#[derive(Args, Debug)]
pub(crate) struct BuildArgs {
    /// 入力ファイル(.cppのみ対応)
    #[arg(required = true, value_hint(ValueHint::FilePath))]
    file: PathBuf,

    /// 出力先ファイル
    #[arg(short = 'o', long, value_hint(ValueHint::FilePath))]
    output: Option<PathBuf>,

    /// コンパイル最適化フラグ
    #[arg(long, default_value_t = false)]
    release: bool,
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
pub(crate) fn build(args: &BuildArgs) -> Result<()> {
    log::info!("{}\n{:?}", style("Build program").bold().green(), args);

    check_args(args)?;

    let mut command = CompileCommand::load_config(if args.release {
        CompileMode::Release
    } else {
        CompileMode::Debug
    })?;
    command.src = Some(args.file.clone().into());
    command.dst = Some(match &args.output {
        Some(s) => PathBuf::from(s),
        None => PathBuf::from(&args.file).with_extension("exe"),
    });

    command.exec_compile()?;

    log::info!(
        "{}\nInput: {}\nOutput: {}",
        style("Build completed").bold().green(),
        command.src.unwrap().display(),
        command.dst.unwrap().display()
    );
    Ok(())
}

fn check_args(args: &BuildArgs) -> Result<()> {
    ensure!(
        args.file.exists(),
        "Input File {} not found.",
        args.file.to_string_lossy()
    );
    ensure!(
        args.file.extension().unwrap() == "cpp",
        "Only .cpp file is supported."
    );
    Ok(())
}
