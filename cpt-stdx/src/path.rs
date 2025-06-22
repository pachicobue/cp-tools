use std::path::{Path, PathBuf};

/// Extracts the file stem (filename without extension) from a path.
///
/// # Arguments
///
/// * `path` - Path to extract the file stem from
///
/// # Returns
///
/// The file stem as a String, or empty string if the path has no file stem
///
/// # Example
///
/// ```rust
/// use cpt_stdx::path::get_filestem;
///
/// assert_eq!(get_filestem("test.txt"), "test");
/// assert_eq!(get_filestem("/path/to/file.rs"), "file");
/// assert_eq!(get_filestem("directory/"), "directory");
/// ```
pub fn get_filestem<T: AsRef<Path>>(path: T) -> String {
    path.as_ref()
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into()
}

/// Extracts the parent directory from a path.
///
/// # Arguments
///
/// * `path` - Path to extract the parent directory from
///
/// # Returns
///
/// The parent directory as a PathBuf, or empty path if no parent exists
///
/// # Example
///
/// ```rust
/// use cpt_stdx::path::get_basedir;
/// use std::path::Path;
///
/// assert_eq!(get_basedir("dir/file.txt"), Path::new("dir"));
/// assert_eq!(get_basedir("/absolute/path/file.txt"), Path::new("/absolute/path"));
/// assert_eq!(get_basedir("file.txt"), Path::new(""));
/// ```
pub fn get_basedir<T: AsRef<Path>>(path: T) -> PathBuf {
    path.as_ref().parent().unwrap_or(Path::new("")).into()
}

/// Extracts the file extension from a path.
///
/// # Arguments
///
/// * `path` - Path to extract the extension from
///
/// # Returns
///
/// The file extension as a String, or empty string if the path has no extension
///
/// # Example
///
/// ```rust
/// use cpt_stdx::path::get_extension;
///
/// assert_eq!(get_extension("test.txt"), "txt");
/// assert_eq!(get_extension("/path/to/file.rs"), "rs");
/// assert_eq!(get_extension("no_extension"), "");
/// ```
pub fn get_extension<T: AsRef<Path>>(path: T) -> String {
    path.as_ref()
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .into()
}

#[cfg(test)]
mod tests {
    use super::{get_basedir, get_extension, get_filestem};
    use pretty_assertions::assert_eq;
    use std::path::Path;
    use test_case::test_case;

    #[test_case("dir1/dir2/dir3/", "Test.txt"; "relative filepath")]
    #[test_case("/dir1/dir2/dir3/", "Test.txt"; "absolute filepath")]
    #[test_case("/", "Test.txt"; "root filepath")]
    #[test_case("", "Test.txt"; "relative-samedir filepath")]
    fn basedir(dir_str: &str, filename: &str) {
        let dir = Path::new(dir_str).to_owned();
        let path = dir.join(filename);
        assert_eq!(get_basedir(&path), dir);
    }

    #[test_case("Test", "in"; "input filepath")]
    #[test_case("Test", "out"; "output filepath")]
    #[test_case("Test", ""; "no ext")]
    fn filestem(filestem: &str, ext: &str) {
        let dir = assert_fs::TempDir::new().unwrap().to_path_buf();
        let path = dir.join(Path::new(filestem).with_extension(ext));
        assert_eq!(get_filestem(&path), filestem);
    }

    #[test_case("Test", "in"; "input filepath")]
    #[test_case("Test", "out"; "output filepath")]
    #[test_case("Test", ""; "no ext")]
    fn extension(filestem: &str, ext: &str) {
        let dir = assert_fs::TempDir::new().unwrap().to_path_buf();
        let path = dir.join(Path::new(filestem).with_extension(ext));
        assert_eq!(get_extension(&path), ext);
    }
}
