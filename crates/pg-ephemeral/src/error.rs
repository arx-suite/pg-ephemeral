use crate::containerized::ContainerizedError;
use crate::local::LocalError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid password configuration: {0}")]
    PasswordMethodFailed(String),

    #[cfg(feature = "local")]
    #[error("local error: {0}")]
    LocalError(#[from] LocalError),

    #[cfg(feature = "containerized")]
    #[error("containerized error: {0}")]
    ContainerizedError(#[from] ContainerizedError),
}
