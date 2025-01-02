use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub struct PathInfo {
    pub exists: bool,
    pub is_file: bool,
    pub is_dir: bool,
    pub basedir: PathBuf,
    pub filestem: String,
    pub extension: String,
}
impl PathInfo {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let mut path_info = PathInfo {
            exists: path.as_ref().exists(),
            is_file: path.as_ref().is_file(),
            is_dir: path.as_ref().is_dir(),
            ..Default::default()
        };
        if path_info.is_dir {
            path_info.basedir = path.as_ref().into();
        } else if path_info.is_file {
            path_info.basedir = path.as_ref().parent().unwrap_or(Path::new("")).into();
            path_info.filestem = path
                .as_ref()
                .file_stem()
                .unwrap_or(OsStr::new(""))
                .to_string_lossy()
                .into();
            path_info.extension = path
                .as_ref()
                .extension()
                .unwrap_or(OsStr::new(""))
                .to_string_lossy()
                .into();
        }
        path_info
    }
}
