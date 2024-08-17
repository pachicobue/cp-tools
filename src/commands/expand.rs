pub mod beautify;
pub mod dummy_headers;

use std::path::{Path, PathBuf};

use clap::{Args, ValueHint};
use color_eyre::eyre::{ensure, OptionExt, Result};

use self::beautify::beautify;
use crate::{
    commands::compilation::{CompileCommand, CompileMode},
    config::dirs::project_workdir,
    core::fs::{filename, write_sync},
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
pub(crate) fn expand(args: &ExpandArgs) -> Result<()> {
    log::info!("{}\n{:?}", styled!("Expand program").bold().green(), args);
    check_args(args)?;

    let dummy_header_dir = dummy_headers::generate()?;

    let dst = match &args.output {
        Some(s) => PathBuf::from(&s),
        None => default_output_path(&args.file)?,
    };
    let mut command = CompileCommand::load_config(CompileMode::Expand)?;
    // command.include_dirs.push(dummy_header_dir);
    let output = command.exec_compilation(&args.file, None)?;

    // write_sync(&dst, beautify(&output)?, true)?;
    write_sync(&dst, &output, true)?;

    log::info!(
        "{}\nInput : {}\nOutput: {}",
        styled!("Expand completed").bold().green(),
        args.file.display(),
        dst.display()
    );
    Ok(())
}

fn check_args(args: &ExpandArgs) -> Result<()> {
    ensure!(
        args.file.exists(),
        "Input File {} not found.",
        args.file.to_string_lossy()
    );
    ensure!(
        args.file.extension().ok_or_eyre("Failed to get ext.")? == "cpp",
        "Only .cpp files are supported."
    );
    Ok(())
}

fn default_output_path(filepath: &Path) -> Result<PathBuf> {
    let basedir = project_workdir(filepath.parent().ok_or_eyre("Failed to get parent.")?)?;
    Ok(basedir.join(filename(filepath)? + "_bundled.cpp"))
}
