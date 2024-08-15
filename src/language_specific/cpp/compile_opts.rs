use std::{fs, path::PathBuf};

use color_eyre::eyre::Result;
use console::Term;
use serde::{Deserialize, Serialize};

use crate::{config::dirs::tool_workdir, styled};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CompOpts {
    /// コンパイラ
    pub compiler: String,
    /// インクルードパス
    pub include_directories: Vec<String>,
    /// マクロ
    pub macros: Vec<String>,
    /// コンパイルオプション（共通）
    pub common_opts: Vec<String>,
    /// コンパイルオプション（デバッグ）
    pub debug_opts: Vec<String>,
    /// コンパイルオプション（リリース）
    pub release_opts: Vec<String>,
}

pub(crate) fn load_opts() -> Result<CompOpts> {
    let path = config_path()?;
    let opts: CompOpts = serde_json::from_reader(fs::File::open(path)?)?;
    Ok(opts)
}

fn config_path() -> Result<PathBuf> {
    let path = tool_workdir().join("compile_commands.json");
    if !path.exists() {
        log::warn!(
                "{}",
                styled!(
                    "{} does not exist!\nCreated with default settings. Please edit it!\nPress any key...",
                    path.to_string_lossy()
                )
                .yellow()
            );
        Term::stdout().read_char()?;
        let content = include_str!("../../../resources/cpp/compile_commands.json");
        fs::write(&path, &content)?;
    }
    Ok(path)
}
