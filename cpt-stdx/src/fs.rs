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

#[cfg(test)]
mod tests {
    use super::{create, open, read, write};
    use assert_fs::prelude::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn read_ok() {
        let file = assert_fs::NamedTempFile::new("test.txt").unwrap();
        file.write_str("Test!").unwrap();
        let res = read(file);
        assert!(res.is_ok_and(|content| content == "Test!"));
    }
    #[test]
    fn read_error() {
        let file = assert_fs::TempDir::new().unwrap().join("phantom.txt");
        let res = read(file);
        assert!(res.is_err());
    }
    #[test]
    fn write_ok() {
        let file = assert_fs::NamedTempFile::new("test.txt").unwrap();
        let res = write(&file, "Test!", false);
        assert!(res.is_ok());
        assert_eq!(read(file).unwrap(), "Test!");
    }
    #[test]
    fn write_err() {
        let file = assert_fs::TempDir::new().unwrap().join("phantom.txt");
        let res = write(&file, "Test!", false);
        assert!(res.is_err());
    }
    #[test]
    fn write_ensure_exist() {
        let file = assert_fs::TempDir::new()
            .unwrap()
            .join("subdir")
            .join("phantom.txt");
        let res = write(&file, "Test!", true);
        assert!(res.is_ok());
        assert!(file.exists());
        assert_eq!(read(file).unwrap(), "Test!");
    }
    #[test]
    fn open_ok() {
        let file = assert_fs::NamedTempFile::new("test.txt").unwrap();
        file.write_str("Read Test!").unwrap();
        let res = open(file);
        assert!(res.is_ok());
    }
    #[test]
    fn open_error() {
        let file = assert_fs::TempDir::new().unwrap().join("phantom.txt");
        let res = open(file);
        assert!(res.is_err());
    }
    #[test]
    fn create_ok() {
        let file = assert_fs::NamedTempFile::new("test.txt").unwrap();
        let res = create(&file, false);
        assert!(res.is_ok());
    }
    #[test]
    fn create_err() {
        let file = assert_fs::TempDir::new().unwrap().join("phantom.txt");
        let res = create(&file, false);
        assert!(res.is_err());
    }
    #[test]
    fn create_ensure_exist() {
        let file = assert_fs::TempDir::new()
            .unwrap()
            .join("subdir")
            .join("phantom.txt");
        let res = create(&file, true);
        assert!(res.is_ok());
        assert!(file.exists());
    }
}
