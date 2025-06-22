use std::path::Path;

use tempfile::{self, TempDir};

pub(crate) const CRATE_NAME: &str = "cpp_bundle";

pub(crate) fn with_tempdir<F, R>(func: F) -> R
where
    F: FnOnce(&TempDir) -> R,
{
    let tempdir = tempfile::Builder::new()
        .prefix(&format!("{}-test-", CRATE_NAME))
        .tempdir()
        .unwrap();
    let result = func(&tempdir);
    tempdir.close().unwrap();
    result
}

pub(crate) fn write_file(
    filepath: impl AsRef<Path>,
    content: impl AsRef<[u8]>,
    ensure_exist: bool,
) -> () {
    if ensure_exist {
        let dir = filepath.as_ref().parent().unwrap();
        std::fs::create_dir_all(dir).unwrap();
    }
    std::fs::write(filepath.as_ref(), &content).unwrap();
}

pub(crate) fn read_file(filepath: impl AsRef<Path>) -> String {
    std::fs::read_to_string(filepath).unwrap()
}

pub(crate) fn has_compiler() -> bool {
    which::which("clang++").is_ok() || which::which("g++").is_ok()
}
