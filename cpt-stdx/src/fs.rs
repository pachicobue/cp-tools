use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not read file `{0}`.")]
    ReadFailed(PathBuf),
    #[error("Could not write file `{0}`.")]
    WriteFailed(PathBuf),
    #[error("Could not open file `{0}`.")]
    OpenFailed(PathBuf),
    #[error("Could not create file `{0}`.")]
    CreateFileFailed(PathBuf),
    #[error("Could not create directory `{0}`.")]
    CreateDirFailed(PathBuf),
}

pub fn read(filepath: impl AsRef<Path>) -> Result<String, Error> {
    let file = filepath.as_ref();
    std::fs::read_to_string(file).map_err(|_| Error::ReadFailed(file.into()))
}

pub fn write(
    filepath: impl AsRef<Path>,
    content: impl AsRef<[u8]>,
    ensure_exist: bool,
) -> Result<(), Error> {
    let file = filepath.as_ref();
    if ensure_exist {
        let dir = file.parent().unwrap();
        std::fs::create_dir_all(dir).map_err(|_| Error::CreateDirFailed(dir.into()))?;
    }
    std::fs::write(file, &content).map_err(|_| Error::WriteFailed(file.into()))
}

pub fn open(filepath: impl AsRef<Path>) -> Result<std::fs::File, Error> {
    let file = filepath.as_ref();
    std::fs::File::open(file).map_err(|_| Error::OpenFailed(file.into()))
}

pub fn create(filepath: impl AsRef<Path>, ensure_exist: bool) -> Result<std::fs::File, Error> {
    let file = filepath.as_ref();
    if ensure_exist {
        let dir = file.parent().unwrap();
        std::fs::create_dir_all(dir).map_err(|_| Error::CreateDirFailed(dir.into()))?;
    }
    std::fs::File::create(file).map_err(|_| Error::CreateFileFailed(file.into()))
}
