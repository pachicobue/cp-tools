use color_eyre::eyre;
use thiserror::Error;

use crate::{commands::Command, core::process::CommandExpression};

#[derive(Error, Debug)]
pub(crate) enum AppError {
    #[error("Invalid argments of `{command}`. {detail}")]
    InvalidArgs { command: Command, detail: String },
    #[error("`{command}` failed. {detail}")]
    Failure { command: Command, detail: String },
    #[error("External command `{}` failed. {detail}", expr.to_string())]
    ExternalCommandFailed {
        expr: CommandExpression,
        detail: String,
    },
    #[error("Filesystem error occurred. {0}")]
    FileSystemError(std::io::Error),
    #[error("Network error occurred. {0}")]
    NetworkError(std::io::Error),
    #[error("Fork operation error occurred. {0}")]
    ForkError(std::io::Error),
    #[error("Tokio-runtime error occurred. {0}")]
    TokioRuntimeError(std::io::Error),
    #[error("Unexpected behavier! Checkout implementation!: {0}")]
    Unexpected(#[from] eyre::Error),
}

pub(crate) type AppResult<T> = Result<T, AppError>;
