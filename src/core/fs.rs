use std::path::Path;

use tempfile::{self, TempDir};
use tokio;

use super::error::FilesystemError;
use crate::config::metadata::CRATE_NAME;

pub(crate) fn read_sync(filepath: impl AsRef<Path>) -> Result<String, FilesystemError> {
    let content = std::fs::read_to_string(filepath.as_ref())
        .map_err(|_| FilesystemError::ReadFileError(filepath.as_ref().to_path_buf()))?;
    Ok(content)
}

pub(crate) fn write_sync(
    filepath: impl AsRef<Path>,
    content: impl AsRef<[u8]>,
    ensure_exist: bool,
) -> Result<(), FilesystemError> {
    if ensure_exist {
        let dir = filepath.as_ref().parent().unwrap();
        std::fs::create_dir_all(dir)
            .map_err(|_| FilesystemError::CreateDirError(dir.to_path_buf()))?;
    }
    std::fs::write(filepath.as_ref(), &content)
        .map_err(|_| FilesystemError::WriteFileError(filepath.as_ref().to_path_buf()))?;
    Ok(())
}

pub(crate) fn open_sync(filepath: impl AsRef<Path>) -> Result<std::fs::File, FilesystemError> {
    std::fs::File::open(filepath.as_ref())
        .map_err(|_| FilesystemError::OpenFileError(filepath.as_ref().to_path_buf()))
}

pub(crate) fn create_sync(filepath: impl AsRef<Path>) -> Result<std::fs::File, FilesystemError> {
    std::fs::File::create(filepath.as_ref())
        .map_err(|_| FilesystemError::CreateDirError(filepath.as_ref().to_path_buf()))
}

pub(crate) async fn read_async(filepath: impl AsRef<Path>) -> Result<String, FilesystemError> {
    let content = tokio::fs::read_to_string(filepath.as_ref())
        .await
        .map_err(|_| FilesystemError::ReadFileError(filepath.as_ref().to_path_buf()))?;
    Ok(content)
}

pub(crate) async fn write_async(
    filepath: impl AsRef<Path>,
    content: impl AsRef<[u8]>,
    ensure_exist: bool,
) -> Result<(), FilesystemError> {
    if ensure_exist {
        let dir = filepath.as_ref().parent().unwrap();
        tokio::fs::create_dir_all(dir)
            .await
            .map_err(|_| FilesystemError::CreateDirError(dir.to_path_buf()))?;
    }
    tokio::fs::write(filepath.as_ref(), &content)
        .await
        .map_err(|_| FilesystemError::WriteFileError(filepath.as_ref().to_path_buf()))?;
    Ok(())
}

pub(crate) async fn open_async(
    filepath: impl AsRef<Path>,
) -> Result<tokio::fs::File, FilesystemError> {
    tokio::fs::File::open(filepath.as_ref())
        .await
        .map_err(|_| FilesystemError::OpenFileError(filepath.as_ref().to_path_buf()))
}

pub(crate) async fn create_async(
    filepath: impl AsRef<Path>,
) -> Result<tokio::fs::File, FilesystemError> {
    tokio::fs::File::create(filepath.as_ref())
        .await
        .map_err(|_| FilesystemError::CreateDirError(filepath.as_ref().to_path_buf()))
}

pub(crate) fn filename(filepath: impl AsRef<Path>) -> String {
    filepath
        .as_ref()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

pub(crate) fn with_tempdir<F, R>(func: F) -> R
where
    F: FnOnce(&TempDir) -> R,
{
    let tempdir = tempfile::Builder::new()
        .prefix(&format!("{}-", CRATE_NAME))
        .tempdir()
        .unwrap();
    let result = func(&tempdir);
    tempdir.close().unwrap();
    result
}
