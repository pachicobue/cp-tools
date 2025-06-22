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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_with_tempdir_basic_usage() {
        let result = with_tempdir(|tempdir| {
            assert!(tempdir.path().exists());
            assert!(tempdir.path().is_dir());
            42
        });
        assert_eq!(result, 42);
    }

    #[test]
    fn test_with_tempdir_file_operations() {
        with_tempdir(|tempdir| {
            let test_file = tempdir.path().join("test.txt");
            fs::write(&test_file, "hello world").unwrap();
            
            assert!(test_file.exists());
            let content = fs::read_to_string(&test_file).unwrap();
            assert_eq!(content, "hello world");
        });
    }

    #[test]
    fn test_with_tempdir_cleanup() {
        let temp_path = with_tempdir(|tempdir| {
            let path = tempdir.path().to_path_buf();
            assert!(path.exists());
            path
        });
        
        // After the function returns, the tempdir should be cleaned up
        // Note: This test might be flaky depending on the OS cleanup timing
        // but it's a reasonable check for most cases
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(!temp_path.exists() || !temp_path.is_dir());
    }

    #[test]
    fn test_with_tempdir_prefix() {
        with_tempdir(|tempdir| {
            let dir_name = tempdir.path().file_name().unwrap().to_str().unwrap();
            assert!(dir_name.starts_with("cpt-"));
        });
    }

    #[test]
    fn test_with_tempdir_multiple_calls() {
        let path1 = with_tempdir(|tempdir| tempdir.path().to_path_buf());
        let path2 = with_tempdir(|tempdir| tempdir.path().to_path_buf());
        
        // Each call should create a different tempdir
        assert_ne!(path1, path2);
    }

    #[test]
    fn test_with_tempdir_nested_directories() {
        with_tempdir(|tempdir| {
            let nested = tempdir.path().join("nested").join("directory");
            fs::create_dir_all(&nested).unwrap();
            
            assert!(nested.exists());
            assert!(nested.is_dir());
            
            let test_file = nested.join("test.txt");
            fs::write(&test_file, "nested content").unwrap();
            assert!(test_file.exists());
        });
    }
}
