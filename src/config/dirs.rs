use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use color_eyre::eyre::Result;
use dirs_next;

use crate::config::metadata::CRATE_NAME;

pub(crate) fn tool_workdir() -> PathBuf {
    dirs_next::data_local_dir().unwrap().join(CRATE_NAME)
}

pub fn project_workdir(dir: &Path) -> Result<PathBuf> {
    let path = dir.join(".".to_string() + CRATE_NAME);
    if !path.exists() {
        create_dir_all(&path)?;
    }
    Ok(path)
}

pub(crate) fn init() -> Result<()> {
    create_dir_all(tool_workdir())?;
    Ok(())
}
