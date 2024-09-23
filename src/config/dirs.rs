use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use dirs_next;
use include_dir::{include_dir, Dir, DirEntry};

use crate::{config::metadata::CRATE_NAME, core::fs::write_sync, styled};

pub(crate) fn tool_workdir() -> PathBuf {
    dirs_next::data_local_dir().unwrap().join(CRATE_NAME)
}

/// プロジェクトの作業ディレクトリを取得する関数
///
/// # 引数
///
/// * `dir` - プロジェクトのディレクトリ
///
/// # 戻り値
///
/// プロジェクトの作業ディレクトリのパス
pub fn project_workdir(dir: &Path) -> PathBuf {
    let path = dir.join(".".to_string() + CRATE_NAME);
    if !path.exists() {
        create_dir_all(&path).unwrap();
    }
    path
}

/// プロジェクトの作業ディレクトリを初期化する関数
///
/// # 引数
///
/// * `dir` - プロジェクトのディレクトリ
///
/// # 戻り値
///
/// なし    
pub(crate) fn init() {
    log::debug!("Creating workspace directory...");
    let workdir = tool_workdir();
    if !workdir.exists() {
        log::info!("Create workspace directory.");
        create_dir_all(workdir).unwrap();
    }

    log::debug!("Copying resources content...");
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");
    let dst_dir = tool_workdir();
    let resource_dir = include_dir!("$CARGO_MANIFEST_DIR/resources");
    copy_resources(&resource_dir, &src_dir, &dst_dir);
}

/// リソースディレクトリの内容をコピーする関数
///
/// # 引数
///
/// * `dir` - リソースディレクトリ
/// * `src_dir` - ソースディレクトリ
/// * `dst_dir` - ディレクトリ
///
/// # 戻り値
///
/// なし    
fn copy_resources(dir: &Dir, src_dir: &Path, dst_dir: &Path) {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(subdir) => copy_resources(subdir, src_dir, dst_dir),
            DirEntry::File(file) => {
                let path = file.path().to_path_buf();
                let dst_path = dst_dir.join(path);
                if !dst_path.exists() {
                    log::warn!(
                        "{}",
                        styled!(
                            "{} does not exist!\nCreated with default settings. Please edit it!",
                            dst_path.display()
                        )
                        .yellow()
                    );
                    write_sync(dst_path, &file.contents(), true).unwrap();
                }
            }
        }
    }
}
