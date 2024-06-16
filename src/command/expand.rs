use crate::{
    config,
    utility::compile::{CompileCommand, CompileMode},
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use ::{
    anyhow::{ensure, Result},
    clap::Args,
};

#[derive(Args, Debug)]
pub(crate) struct ExpandArgs {
    /// 入力ファイル(.cppのみ対応)
    #[arg(required = true)]
    file: String,

    /// 出力先ファイル
    #[arg(short = 'o', long)]
    output: Option<String>,
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
pub(crate) async fn expand(args: &ExpandArgs) -> Result<()> {
    log::info!("Expanding {} ...", args.file);
    check_args(args)?;

    let dst = match &args.output {
        Some(s) => PathBuf::from(&s),
        None => default_output(),
    };
    let mut command = CompileCommand::load_config(CompileMode::Expand)?;
    command.src = Some(args.file.clone().into());
    let output = "#include <bits/stdc++.h>\n".to_string() + &command.exec_compile().await?;
    fs::write(dst.clone(), output)?;

    log::info!("Expanded to {} .", dst.display());
    Ok(())
}

fn default_output() -> PathBuf {
    config::dirs::workspace_dir().join("Bundled.cpp")
}

fn check_args(args: &ExpandArgs) -> Result<()> {
    ensure!(
        Path::new(&args.file).exists(),
        "Input File {} not found.",
        args.file
    );
    ensure!(
        args.file.ends_with(".cpp"),
        "Only .cpp files are supported."
    );
    Ok(())
}
