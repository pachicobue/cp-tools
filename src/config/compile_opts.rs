use crate::config;
use std::{fs, path::PathBuf};
use ::{
    anyhow::Result,
    serde::{Deserialize, Serialize},
};

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

impl CompOpts {
    fn default() -> Self {
        CompOpts {
            compiler: "clang++".to_string(),
            include_directories: [
                "/home/sho/ghq/github.com/pachicobue/algolib/src",
                "/home/sho/ghq/github.com/atcoder/ac-library",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            macros: ["HOGEPACHI"].iter().map(|s| s.to_string()).collect(),
            common_opts: ["-std=gnu++20", "-Wall", "-Wextra"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            debug_opts: ["-g3", "-O0", "-fsanitize=undefined,address"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            release_opts: ["-O2"].iter().map(|s| s.to_string()).collect(),
        }
    }
}

pub(crate) fn init() -> Result<()> {
    let path = config_path();
    if path.exists() {
        return Ok(());
    }
    let opt = CompOpts::default();
    let json = serde_json::to_string_pretty(&opt)?;
    fs::write(&path, json.as_bytes())?;
    log::warn!(
        "{} created with default settings. Please edit it.",
        path.to_string_lossy()
    );
    Ok(())
}

pub(crate) fn load_opts() -> Result<CompOpts> {
    let path = config_path();
    let opts: CompOpts = serde_json::from_reader(fs::File::open(path)?)?;
    Ok(opts)
}

fn config_path() -> PathBuf {
    config::dirs::workspace_dir().join("compile_commands.json")
}
