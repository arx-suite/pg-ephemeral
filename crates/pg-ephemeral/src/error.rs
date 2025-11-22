use crate::ephemeral::BuilderError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O operation failed: {0}")]
    IOError(#[from] std::io::Error),

    #[error("program not found in $PATH")]
    ProgramNotFound,

    #[error("invalid password configuration: {0}")]
    PasswordMethodFailed(String),

    #[error("failed to construct `PgEphemeral`: {0}")]
    BuilderError(#[from] BuilderError),
}
