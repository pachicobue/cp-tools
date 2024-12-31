use std::str::from_utf8;

use thiserror::Error;

use crate::core::fs::{self, FilesystemError};
use crate::dir;

pub(crate) fn load_tomls() -> Vec<toml::Value> {
    let mut tomls = vec![];
    tomls.push(from_default());
    if let Ok(toml) = from_datadir() {
        tomls.push(toml);
    }
    if let Ok(toml) = from_workspace() {
        tomls.push(toml);
    }
    tomls
}

#[derive(Error, Debug)]
enum TomlLoadError {
    #[error(transparent)]
    CannotReadFile(#[from] FilesystemError),
    #[error(transparent)]
    ParseFailed(#[from] toml::de::Error),
}

/// Default toml
fn from_default() -> toml::Value {
    let default_toml = include_bytes!("../../../languages.toml");
    toml::from_str(from_utf8(default_toml).unwrap())
        .expect("Failed to parse default languages.toml!")
}

/// Datadir toml
fn from_datadir() -> Result<toml::Value, TomlLoadError> {
    let toml_str = fs::read_sync(dir::data_dir().join("languages.toml"))?;
    toml::from_str(&toml_str).map_err(|e| TomlLoadError::ParseFailed(e))
}

/// Workspace toml
fn from_workspace() -> Result<toml::Value, TomlLoadError> {
    let toml_str = fs::read_sync(dir::workspace_dir().join("languages.toml"))?;
    toml::from_str(&toml_str).map_err(|e| TomlLoadError::ParseFailed(e))
}
