use cpt_stdx::{fs::FilesystemError, task::TaskError};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum BatchTestError {
    #[error("Failed to execute program")]
    FailedToExecuteProgram(TaskError),
    #[error("Failed to read file for `input`")]
    ReadInputError(FilesystemError),
    #[error("Failed to read file for `expect`")]
    ReadExpectError(FilesystemError),
    #[error("Failed to write file for `actual`")]
    WriteActualError(FilesystemError),
}
