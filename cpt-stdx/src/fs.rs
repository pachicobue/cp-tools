use std::path::{Path, PathBuf};

/// Error types for file system operations.
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

/// Reads the entire contents of a file as a UTF-8 string.
///
/// # Arguments
///
/// * `filepath` - Path to the file to read
///
/// # Returns
///
/// * `Ok(String)` - The file contents as a UTF-8 string
/// * `Err(Error::Read)` - If the file cannot be read or is not valid UTF-8
///
/// # Example
///
/// ```no_run
/// use cpt_stdx::fs;
///
/// // Create a temporary file for the example
/// let temp_file = std::env::temp_dir().join("example.txt");
/// std::fs::write(&temp_file, "Hello, world!").unwrap();
///
/// let content = fs::read(&temp_file).expect("Failed to read file");
/// assert_eq!(content, "Hello, world!");
///
/// // Clean up
/// std::fs::remove_file(&temp_file).unwrap();
/// ```
pub fn read(filepath: impl AsRef<Path>) -> Result<String, Error> {
    let file = filepath.as_ref();
    std::fs::read_to_string(file).map_err(|_| Error::Read(file.into()))
}

/// Writes content to a file.
///
/// # Arguments
///
/// * `filepath` - Path to the file to write
/// * `content` - Content to write to the file
/// * `ensure_exist` - If true, creates parent directories if they don't exist
///
/// # Returns
///
/// * `Ok(())` - If the file was written successfully
/// * `Err(Error::Write)` - If the file cannot be written
/// * `Err(Error::CreateDir)` - If parent directory creation fails when `ensure_exist` is true
///
/// # Example
///
/// ```no_run
/// use cpt_stdx::fs;
///
/// // Write to existing directory
/// fs::write("test.txt", "Hello, world!", false).expect("Failed to write file");
///
/// // Create parent directories if needed
/// fs::write("new/dir/test.txt", "Hello, world!", true).expect("Failed to write file");
/// ```
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

/// Opens a file for reading.
///
/// # Arguments
///
/// * `filepath` - Path to the file to open
///
/// # Returns
///
/// * `Ok(File)` - The opened file handle
/// * `Err(Error::Open)` - If the file cannot be opened
///
/// # Example
///
/// ```no_run
/// use cpt_stdx::fs;
/// use std::io::Read;
///
/// // Create a temporary file for the example
/// let temp_file = std::env::temp_dir().join("example.txt");
/// std::fs::write(&temp_file, "Hello, world!").unwrap();
///
/// let mut file = fs::open(&temp_file).expect("Failed to open file");
/// let mut content = String::new();
/// file.read_to_string(&mut content).expect("Failed to read file");
/// assert_eq!(content, "Hello, world!");
///
/// // Clean up
/// std::fs::remove_file(&temp_file).unwrap();
/// ```
pub fn open(filepath: impl AsRef<Path>) -> Result<std::fs::File, Error> {
    let file = filepath.as_ref();
    std::fs::File::open(file).map_err(|_| Error::Open(file.into()))
}

/// Creates a file for writing.
///
/// # Arguments
///
/// * `filepath` - Path to the file to create
/// * `ensure_exist` - If true, creates parent directories if they don't exist
///
/// # Returns
///
/// * `Ok(File)` - The created file handle
/// * `Err(Error::Create)` - If the file cannot be created
/// * `Err(Error::CreateDir)` - If parent directory creation fails when `ensure_exist` is true
///
/// # Example
///
/// ```no_run
/// use cpt_stdx::fs;
/// use std::io::Write;
///
/// let mut file = fs::create("new_file.txt", false).expect("Failed to create file");
/// file.write_all(b"Hello, world!").expect("Failed to write to file");
/// ```
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
