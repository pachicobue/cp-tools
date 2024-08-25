use std::path::PathBuf;

use thiserror::Error;
use toml;

use super::language::Language;

#[derive(Error, Debug)]
pub(crate) enum ApplicationError {
    #[error("InitializationError")]
    InitializationError(#[from] InitializationError),
    #[error("CommandError")]
    CommandError(#[from] CommandError),
}
#[derive(Error, Debug)]
pub(crate) enum InitializationError {
    #[error("LanguageConfigurationError")]
    LanguageConfigurationError(#[from] LanguageConfigurationError),
}
#[derive(Error, Debug)]
pub(crate) enum LanguageConfigurationError {
    #[error("Error occurred during parsing `{}`.", .1.display())]
    TomlParseError(#[source] toml::de::Error, PathBuf),
    #[error("Language `{1}` is not supported. Please add name to `enum Language`.")]
    LanguageNotSupportedError(#[source] strum::ParseError, String),
    #[error("Same extension `{0}` is specified in multiple languages.")]
    DuplicateExtensionError(String),
}

#[derive(Error, Debug)]
pub(crate) enum CommandError {
    #[error("TestCommandError")]
    TestCommandError(#[from] TestCommandError),
    #[error("BuildCommandError")]
    BuildCommandError(#[from] BuildCommandError),
    #[error("ExpandCommandError")]
    ExpandCommandError(#[from] ExpandCommandError),
}

#[derive(Error, Debug)]
pub(crate) enum TestCommandError {
    #[error("TestArgumentError")]
    TestArgumentError(#[from] TestArgumentError),
    #[error("Testcase not found.")]
    TestCaseNotFound,
}
#[derive(Error, Debug)]
pub(crate) enum TestArgumentError {
    #[error("Testcase directory `{0}` is not found.")]
    CasedirIsNotFound(PathBuf),
    #[error("Testcase path `{0}` is not a directory.")]
    CasedirIsNotDirectory(PathBuf),
}

#[derive(Error, Debug)]
pub(crate) enum BuildCommandError {
    #[error("TestArgumentError")]
    BuildArgumentError(#[from] BuildArgumentError),
    #[error("Build command failed")]
    BuildCommandError,
}
#[derive(Error, Debug)]
pub(crate) enum BuildArgumentError {
    #[error("Src file `{0}` is not found.")]
    SourcefileIsNotFound(PathBuf),
    #[error("Src path `{0}` is not a file.")]
    SourcefileIsNotFile(PathBuf),
    #[error("LanguageSpecificationError")]
    LanguageSpecificationError(#[from] LanguageSpecificationError),
}

#[derive(Error, Debug)]
pub(crate) enum ExpandCommandError {
    #[error("TestArgumentError")]
    ExpandArgumentError(#[from] ExpandArgumentError),
    #[error("FilesystemError")]
    FileSystemError(#[from] FilesystemError),
    #[error("Expand command failed")]
    ExpandCommandError,
}
#[derive(Error, Debug)]
pub(crate) enum ExpandArgumentError {
    #[error("Src file `{0}` is not found.")]
    SourcefileIsNotFound(PathBuf),
    #[error("Src path `{0}` is not a file.")]
    SourcefileIsNotFile(PathBuf),
    #[error("LanguageSpecificationError")]
    LanguageSpecificationError(#[from] LanguageSpecificationError),
}

#[derive(Error, Debug)]
pub(crate) enum LanguageSpecificationError {
    #[error("Failed to get extension from `{0}`.")]
    ExtensionIsNotFound(PathBuf),
    #[error("Extention `{0}` is not defined for any language.")]
    LanguageNotDefined(String),
    #[error("Build command is not supported for language `{0}`.")]
    BuildNotSupported(Language),
    #[error("Exec command is not supported for language `{0}`.")]
    ExecNotSupported(Language),
    #[error("Expand command is not supported for language `{0}`.")]
    ExpandNotSupported(Language),
}

#[derive(Error, Debug)]
pub(crate) enum FilesystemError {
    #[error("Failed to read from file `{0}`.")]
    ReadFileError(PathBuf),
    #[error("Failed to write to file `{0}`.")]
    WriteFileError(PathBuf),
    #[error("Failed to create directory `{0}`.")]
    CreateDirError(PathBuf),
    #[error("Failed to open file `{0}`.")]
    OpenFileError(PathBuf),
}

pub(crate) trait ToTraverseErrorMessage: std::error::Error {
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

fn to_traverse_error_message_inner(error: &dyn std::error::Error, messages: &mut Vec<String>) {
    messages.push(error.to_string());
    if let Some(source) = error.source() {
        to_traverse_error_message_inner(source, messages);
    }
}

impl<T: std::error::Error> ToTraverseErrorMessage for T {}
