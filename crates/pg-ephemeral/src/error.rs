use crate::containerized::ContainerizedError;
use crate::local::config::builder::LocalBuilderError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O operation failed: {0}")]
    IOError(#[from] std::io::Error),

    #[error("invalid password configuration: {0}")]
    PasswordMethodFailed(String),

    #[cfg(feature = "local")]
    #[error("failed to construct `Local`: {0}")]
    LocalBuilderError(#[from] LocalBuilderError),

    #[cfg(feature = "containerized")]
    #[error("containerized error: {0}")]
    ContainerizedError(#[from] ContainerizedError),
}
