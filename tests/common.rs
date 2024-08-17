use std::path::Path;

use tempfile::{self, TempDir};

pub(crate) const CRATE_NAME: &str = "cpt";

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

pub(crate) fn write_sync(
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
