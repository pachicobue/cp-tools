use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FilesystemError {
    #[error("Failed to read from file `{0}`.")]
    ReadFileError(PathBuf),
    #[error("Failed to write to file `{0}`.")]
    WriteFileError(PathBuf),
    #[error("Failed to open file `{0}`.")]
    OpenFileError(PathBuf),
    #[error("Failed to create directory `{0}`.")]
    CreateDirError(PathBuf),
}

pub fn read(filepath: impl AsRef<Path>) -> Result<String, FilesystemError> {
    std::fs::read_to_string(filepath.as_ref())
        .map_err(|_| FilesystemError::ReadFileError(filepath.as_ref().to_path_buf()))
}

pub fn write(
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
