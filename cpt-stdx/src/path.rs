use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub struct PathInfo {
    pub path: PathBuf,
    pub exists: bool,
    pub is_file: bool,
    pub is_dir: bool,
    pub basedir: PathBuf,
    pub filestem: String,
    pub extension: String,
}
impl PathInfo {
    pub fn new<T: AsRef<Path>, S1: AsRef<str>, S2: AsRef<str>>(
        basedir: T,
        filestem: S1,
        extension: S2,
    ) -> Self {
        let basedir = basedir.as_ref();
        let filestem = filestem.as_ref();
        let extension = extension.as_ref();
        let path = basedir
            .join(filestem.to_owned() + (if extension.is_empty() { "" } else { "." }) + extension);
        Self {
            path: path.to_owned(),
            exists: path.exists(),
            is_file: path.is_file(),
            is_dir: path.is_dir(),
            basedir: basedir.into(),
            filestem: filestem.into(),
            extension: extension.into(),
        }
    }
}
impl<T: AsRef<Path>> From<T> for PathInfo {
    fn from(path: T) -> Self {
        let path = path.as_ref();
        let mut path_info = PathInfo {
            path: path.into(),
            exists: path.exists(),
            is_file: path.is_file(),
            is_dir: path.is_dir(),
            ..Default::default()
        };
        if path_info.is_dir {
            path_info.basedir = path.into();
        } else if path_info.is_file {
            path_info.basedir = path.parent().unwrap_or(Path::new("")).into();
            path_info.filestem = path
                .file_stem()
                .unwrap_or(OsStr::new(""))
                .to_string_lossy()
                .into();
            path_info.extension = path
                .extension()
                .unwrap_or(OsStr::new(""))
                .to_string_lossy()
                .into();
        }
        path_info
    }
}
