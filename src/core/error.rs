use std::path::PathBuf;

use thiserror::Error;
use toml;

use super::language::Language;

/// アプリケーションエラーを表す列挙型
#[derive(Error, Debug)]
pub(crate) enum ApplicationError {
    /// 初期化エラー
    #[error("InitializationError")]
    InitializationError(#[from] InitializationError),
    /// コマンドエラー
    #[error("CommandError")]
    CommandError(#[from] CommandError),
}

/// 初期化エラーを表す列挙型
#[derive(Error, Debug)]
pub(crate) enum InitializationError {
    /// 言語設定エラー
    #[error("LanguageConfigurationError")]
    LanguageConfigurationError(#[from] LanguageConfigurationError),
}

/// 言語設定エラーを表す列挙型
#[derive(Error, Debug)]
pub(crate) enum LanguageConfigurationError {
    /// TOML解析エラー
    #[error("Error occurred during parsing `{}`.", .1.display())]
    TomlParseError(#[source] toml::de::Error, PathBuf),
    /// サポートされていない言語エラー
    #[error("Language `{1}` is not supported. Please add name to `enum Language`.")]
    LanguageNotSupportedError(#[source] strum::ParseError, String),
    /// 重複する拡張子エラー
    #[error("Same extension `{0}` is specified in multiple languages.")]
    DuplicateExtensionError(String),
}

/// コマンドエラーを表す列挙型
#[derive(Error, Debug)]
pub(crate) enum CommandError {
    /// テストコマンドエラー
    #[error("TestCommandError")]
    TestCommandError(#[from] TestCommandError),
    /// ビルドコマンドエラー
    #[error("BuildCommandError")]
    BuildCommandError(#[from] BuildCommandError),
    /// 展開コマンドエラー
    #[error("ExpandCommandError")]
    ExpandCommandError(#[from] ExpandCommandError),
}

/// テストコマンドエラーを表す列挙型
#[derive(Error, Debug)]
pub(crate) enum TestCommandError {
    /// テスト引数エラー
    #[error("TestArgumentError")]
    TestArgumentError(#[from] TestArgumentError),
    /// テストケースが見つからないエラー
    #[error("Testcase not found.")]
    TestCaseNotFound,
}

/// テスト引数エラーを表す列挙型
#[derive(Error, Debug)]
pub(crate) enum TestArgumentError {
    /// テストケースディレクトリが見つからないエラー
    #[error("Testcase directory `{0}` is not found.")]
    CasedirIsNotFound(PathBuf),
    /// テストケースパスがディレクトリでないエラー
    #[error("Testcase path `{0}` is not a directory.")]
    CasedirIsNotDirectory(PathBuf),
}

/// ビルドコマンドエラーを表す列挙型
#[derive(Error, Debug)]
pub(crate) enum BuildCommandError {
    /// ビルド引数エラー
    #[error("TestArgumentError")]
    BuildArgumentError(#[from] BuildArgumentError),
    /// ビルドコマンド失敗エラー
    #[error("Build command failed")]
    BuildCommandError,
}

/// ビルド引数エラーを表す列挙型
#[derive(Error, Debug)]
pub(crate) enum BuildArgumentError {
    /// ソースファイルが見つからないエラー
    #[error("Src file `{0}` is not found.")]
    SourcefileIsNotFound(PathBuf),
    /// ソースパスがファイルでないエラー
    #[error("Src path `{0}` is not a file.")]
    SourcefileIsNotFile(PathBuf),
    /// 言語仕様エラー
    #[error("LanguageSpecificationError")]
    LanguageSpecificationError(#[from] LanguageSpecificationError),
}

/// 展開コマンドエラーを表す列挙型
#[derive(Error, Debug)]
pub(crate) enum ExpandCommandError {
    /// 展開引数エラー
    #[error("TestArgumentError")]
    ExpandArgumentError(#[from] ExpandArgumentError),
    /// ファイルシステムエラー
    #[error("FilesystemError")]
    FileSystemError(#[from] FilesystemError),
    /// 展開コマンド失敗エラー
    #[error("Expand command failed")]
    ExpandCommandError,
}

/// 展開引数エラーを表す列挙型
#[derive(Error, Debug)]
pub(crate) enum ExpandArgumentError {
    /// ソースファイルが見つからないエラー
    #[error("Src file `{0}` is not found.")]
    SourcefileIsNotFound(PathBuf),
    /// ソースパスがファイルでないエラー
    #[error("Src path `{0}` is not a file.")]
    SourcefileIsNotFile(PathBuf),
    /// 言語仕様エラー
    #[error("LanguageSpecificationError")]
    LanguageSpecificationError(#[from] LanguageSpecificationError),
}

/// 言語仕様エラーを表す列挙型
#[derive(Error, Debug)]
pub(crate) enum LanguageSpecificationError {
    /// 拡張子が見つからないエラー
    #[error("Failed to get extension from `{0}`.")]
    ExtensionIsNotFound(PathBuf),
    /// 拡張子が定義されていないエラー
    #[error("Extention `{0}` is not defined for any language.")]
    LanguageNotDefined(String),
    /// ビルドコマンドがサポートされていないエラー
    #[error("Build command is not supported for language `{0}`.")]
    BuildNotSupported(Language),
    /// 実行コマンドがサポートされていないエラー
    #[error("Exec command is not supported for language `{0}`.")]
    ExecNotSupported(Language),
    /// 展開コマンドがサポートされていないエラー
    #[error("Expand command is not supported for language `{0}`.")]
    ExpandNotSupported(Language),
}

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

/// エラーメッセージをトラバースして取得するトレイト
pub(crate) trait ToTraverseErrorMessage: std::error::Error {
    /// エラーメッセージをトラバースして取得する関数
    ///
    /// # 戻り値
    ///
    /// トラバースされたエラーメッセージの文字列
    fn to_traverse_error_message(&self) -> String {
        let mut messages = vec![];
        if let Some(source) = self.source() {
            to_traverse_error_message_inner(source, &mut messages)
        }
        let children_message = messages
            .into_iter()
            .map(|message| format!("* {message}"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("{}\n{children_message}", self)
    }
}

/// 内部エラーメッセージをトラバースして取得する関数
///
/// # 引数
///
/// * `error` - エラーオブジェクト
/// * `messages` - メッセージのベクター
fn to_traverse_error_message_inner(error: &dyn std::error::Error, messages: &mut Vec<String>) {
    messages.push(error.to_string());
    if let Some(source) = error.source() {
        to_traverse_error_message_inner(source, messages);
    }
}

/// `ToTraverseErrorMessage`トレイトの実装
impl<T: std::error::Error> ToTraverseErrorMessage for T {}
