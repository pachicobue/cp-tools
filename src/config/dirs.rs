use std::{fs, path::PathBuf};
use ::{anyhow::Result, dirs_next};

pub(crate) fn workspace_dir() -> PathBuf {
    dirs_next::data_local_dir().unwrap().join("cp_tools")
}

pub(crate) fn init() -> Result<()> {
    Ok(fs::create_dir_all(workspace_dir())?)
}
