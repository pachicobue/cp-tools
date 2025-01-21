use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not read file `{0}`.")]
    Read(PathBuf),
    #[error("Could not write file `{0}`.")]
    Write(PathBuf),
    #[error("Could not open file `{0}`.")]
    Open(PathBuf),
    #[error("Could not create file `{0}`.")]
    CreateFile(PathBuf),
    #[error("Could not create directory `{0}`.")]
    CreateDir(PathBuf),
}

pub fn read(filepath: impl AsRef<Path>) -> Result<String, Error> {
    let file = filepath.as_ref();
    std::fs::read_to_string(file).map_err(|_| Error::Read(file.into()))
}

pub fn write(
    filepath: impl AsRef<Path>,
    content: impl AsRef<[u8]>,
    ensure_exist: bool,
) -> Result<(), Error> {
    let file = filepath.as_ref();
    if ensure_exist {
        let dir = file.parent().unwrap();
        std::fs::create_dir_all(dir).map_err(|_| Error::CreateDir(dir.into()))?;
    }
    std::fs::write(file, &content).map_err(|_| Error::Write(file.into()))
}

pub fn open(filepath: impl AsRef<Path>) -> Result<std::fs::File, Error> {
    let file = filepath.as_ref();
    std::fs::File::open(file).map_err(|_| Error::Open(file.into()))
}

pub fn create(filepath: impl AsRef<Path>, ensure_exist: bool) -> Result<std::fs::File, Error> {
    let file = filepath.as_ref();
    if ensure_exist {
        let dir = file.parent().unwrap();
        std::fs::create_dir_all(dir).map_err(|_| Error::CreateDir(dir.into()))?;
    }
    std::fs::File::create(file).map_err(|_| Error::CreateFile(file.into()))
}
