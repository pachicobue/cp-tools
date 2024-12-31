use std::env;
use std::path::PathBuf;
use std::sync::OnceLock;

use etcetera::BaseStrategy;
use thiserror::Error;

static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();
static WORKSPACE_DIR: OnceLock<PathBuf> = OnceLock::new();

#[derive(Error, Debug)]
pub(crate) enum DirError {
    #[error("Failed to get CWD")]
    InvalidCwd(#[source] std::io::Error),
}

fn search_workspace() -> Result<PathBuf, DirError> {
    let cwd = env::current_dir().map_err(|e| DirError::InvalidCwd(e))?;
    for ancestor in cwd.ancestors() {
        if ancestor.join(".cpt").exists() {
            return Ok(ancestor.join(".cpt").to_owned());
        }
    }
    return Ok(cwd.to_owned());
}

/// Data directory path
/// `.config/local/share/cpt`
pub(crate) fn data_dir() -> PathBuf {
    DATA_DIR.get().unwrap().to_path_buf()
}

/// Workspace directory path
/// `Ancestor(CWD)/.cpt` or `CWD`
pub(crate) fn workspace_dir() -> PathBuf {
    WORKSPACE_DIR.get().unwrap().to_path_buf()
}

pub(crate) fn init() -> Result<(), DirError> {
    let strategy = etcetera::choose_base_strategy().expect("Failed to default base strategy.");
    DATA_DIR.set(strategy.data_dir().join("cpt")).unwrap();
    WORKSPACE_DIR.set(search_workspace()?).unwrap();
    log::debug!("DATA_DIR: {}", DATA_DIR.get().unwrap().to_string_lossy());
    log::debug!(
        "WORKSPACE_DIR: {}",
        WORKSPACE_DIR.get().unwrap().to_string_lossy()
    );
    Ok(())
}
