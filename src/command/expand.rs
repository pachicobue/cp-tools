use crate::{
    config,
    utility::{
        beautify::beautify,
        compile::{CompileCommand, CompileMode},
        dummy_headers,
    },
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use ::{
    clap::{Args, ValueHint},
    color_eyre::eyre::{ensure, Result},
    console::style,
    regex::Regex,
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
///
pub(crate) fn expand(args: &ExpandArgs) -> Result<()> {
    log::info!("{}\n{:?}", style("Expand program").bold().green(), args);
    check_args(args)?;

    let dummy_header_dir = dummy_headers::generate()?;

    let dst = match &args.output {
        Some(s) => PathBuf::from(&s),
        None => default_output(),
    };
    let mut command = CompileCommand::load_config(CompileMode::Expand)?;
    command.src = Some(args.file.clone());
    command.include_dirs.push(dummy_header_dir);
    let output = command.exec_compile()?;

    beautify(&output, &dst)?;

    log::info!("End expand: {} -> {} .", args.file.display(), dst.display());
    Ok(())
}

fn default_output() -> PathBuf {
    config::dirs::workspace_dir().join("Bundled.cpp")
}

fn check_args(args: &ExpandArgs) -> Result<()> {
    ensure!(
        args.file.exists(),
        "Input File {} not found.",
        args.file.to_string_lossy()
    );
    ensure!(
        args.file.extension().unwrap() == "cpp",
        "Only .cpp files are supported."
    );
    Ok(())
}
