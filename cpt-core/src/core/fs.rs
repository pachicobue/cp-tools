use std::path::{Path, PathBuf};

use tempfile::{self, TempDir};
use thiserror::Error;
use tokio;

/// ファイルシステムエラーを表す列挙型
#[derive(Error, Debug)]
pub(crate) enum FilesystemError {
    /// ファイルの読み取り失敗エラー
    #[error("Failed to read from file `{0}`.")]
    ReadFileError(PathBuf),
    /// ファイルの書き込み失敗エラー
    #[error("Failed to write to file `{0}`.")]
    WriteFileError(PathBuf),
    /// ディレクトリの作成失敗エラー
    #[error("Failed to create directory `{0}`.")]
    CreateDirError(PathBuf),
    /// ファイルのオープン失敗エラー
    #[error("Failed to open file `{0}`.")]
    OpenFileError(PathBuf),
}

/// 同期的にファイルを読み込む関数
///
/// # 引数
///
/// * `filepath` - 読み込むファイルのパス
///
/// # 戻り値
///
/// ファイルの内容を文字列として返す。エラーが発生した場合は`FilesystemError`を返す。
pub(crate) fn read_sync(filepath: impl AsRef<Path>) -> Result<String, FilesystemError> {
    let content = std::fs::read_to_string(filepath.as_ref())
        .map_err(|_| FilesystemError::ReadFileError(filepath.as_ref().to_path_buf()))?;
    Ok(content)
}

/// 同期的にファイルに書き込む関数
///
/// # 引数
///
/// * `filepath` - 書き込むファイルのパス
/// * `content` - 書き込む内容
/// * `ensure_exist` - ディレクトリが存在しない場合に作成するかどうか
///
/// # 戻り値
///
/// 書き込みに成功した場合は`Ok(())`、失敗した場合は`FilesystemError`を返す。
pub(crate) fn write_sync(
    filepath: impl AsRef<Path>,
    content: impl AsRef<[u8]>,
    ensure_exist: bool,
) -> Result<(), FilesystemError> {
    if ensure_exist {
        let dir = filepath.as_ref().parent().unwrap();
        std::fs::create_dir_all(dir)
            .map_err(|_| FilesystemError::CreateDirError(dir.to_path_buf()))?;
    }
    std::fs::write(filepath.as_ref(), &content)
        .map_err(|_| FilesystemError::WriteFileError(filepath.as_ref().to_path_buf()))?;
    Ok(())
}

/// 同期的にファイルを開く関数
///
/// # 引数
///
/// * `filepath` - 開くファイルのパス
///
/// # 戻り値
///
/// 開いたファイルのハンドルを返す。エラーが発生した場合は`FilesystemError`を返す。
pub(crate) fn open_sync(filepath: impl AsRef<Path>) -> Result<std::fs::File, FilesystemError> {
    std::fs::File::open(filepath.as_ref())
        .map_err(|_| FilesystemError::OpenFileError(filepath.as_ref().to_path_buf()))
}

/// 同期的にファイルを作成する関数
///
/// # 引数
///
/// * `filepath` - 作成するファイルのパス
///
/// # 戻り値
///
/// 作成したファイルのハンドルを返す。エラーが発生した場合は`FilesystemError`を返す。
pub(crate) fn create_sync(filepath: impl AsRef<Path>) -> Result<std::fs::File, FilesystemError> {
    std::fs::File::create(filepath.as_ref())
        .map_err(|_| FilesystemError::CreateDirError(filepath.as_ref().to_path_buf()))
}

/// 非同期的にファイルを読み込む関数
///
/// # 引数
///
/// * `filepath` - 読み込むファイルのパス
///
/// # 戻り値
///
/// ファイルの内容を文字列として返す。エラーが発生した場合は`FilesystemError`を返す。
pub(crate) async fn read_async(filepath: impl AsRef<Path>) -> Result<String, FilesystemError> {
    let content = tokio::fs::read_to_string(filepath.as_ref())
        .await
        .map_err(|_| FilesystemError::ReadFileError(filepath.as_ref().to_path_buf()))?;
    Ok(content)
}

/// 非同期的にファイルに書き込む関数
///
/// # 引数
///
/// * `filepath` - 書き込むファイルのパス
/// * `content` - 書き込む内容
/// * `ensure_exist` - ディレクトリが存在しない場合に作成するかどうか
///
/// # 戻り値
///
/// 書き込みに成功した場合は`Ok(())`、失敗した場合は`FilesystemError`を返す。
pub(crate) async fn write_async(
    filepath: impl AsRef<Path>,
    content: impl AsRef<[u8]>,
    ensure_exist: bool,
) -> Result<(), FilesystemError> {
    if ensure_exist {
        let dir = filepath.as_ref().parent().unwrap();
        tokio::fs::create_dir_all(dir)
            .await
            .map_err(|_| FilesystemError::CreateDirError(dir.to_path_buf()))?;
    }
    tokio::fs::write(filepath.as_ref(), &content)
        .await
        .map_err(|_| FilesystemError::WriteFileError(filepath.as_ref().to_path_buf()))?;
    Ok(())
}

/// 非同期的にファイルを開く関数
///
/// # 引数
///
/// * `filepath` - 開くファイルのパス
///
/// # 戻り値
///
/// 開いたファイルのハンドルを返す。エラーが発生した場合は`FilesystemError`を返す。
pub(crate) async fn open_async(
    filepath: impl AsRef<Path>,
) -> Result<tokio::fs::File, FilesystemError> {
    tokio::fs::File::open(filepath.as_ref())
        .await
        .map_err(|_| FilesystemError::OpenFileError(filepath.as_ref().to_path_buf()))
}

/// 非同期的にファイルを作成する関数
///
/// # 引数
///
/// * `filepath` - 作成するファイルのパス
///
/// # 戻り値
///
/// 作成したファイルのハンドルを返す。エラーが発生した場合は`FilesystemError`を返す。
pub(crate) async fn create_async(
    filepath: impl AsRef<Path>,
) -> Result<tokio::fs::File, FilesystemError> {
    tokio::fs::File::create(filepath.as_ref())
        .await
        .map_err(|_| FilesystemError::CreateDirError(filepath.as_ref().to_path_buf()))
}

/// ファイル名を取得する関数
///
/// # 引数
///
/// * `filepath` - ファイルのパス
///
/// # 戻り値
///
/// ファイル名を文字列として返す。
pub(crate) fn filename(filepath: impl AsRef<Path>) -> String {
    filepath
        .as_ref()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

/// Run function with tempdir
pub(crate) fn with_tempdir<F, R>(func: F) -> R
where
    F: FnOnce(&TempDir) -> R,
{
    let tempdir = tempfile::Builder::new().prefix("cpt-").tempdir().unwrap();
    let result = func(&tempdir);
    tempdir.close().unwrap();
    result
}
