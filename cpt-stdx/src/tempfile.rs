/// Run function with tempdir
pub fn with_tempdir<F, R>(func: F) -> R
where
    F: FnOnce(&tempfile::TempDir) -> R,
{
    let tempdir = tempfile::Builder::new().prefix("cpt-").tempdir().unwrap();
    let result = func(&tempdir);
    tempdir.close().unwrap();
    result
}
