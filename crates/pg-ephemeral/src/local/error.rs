use super::config::LocalBuilderError;

#[derive(Debug, thiserror::Error)]
pub enum LocalError {
    #[error("failed to construct `LocalConfig`: {0}")]
    LocalBuilderError(#[from] LocalBuilderError),

    #[error("program not found: {0}")]
    ProgramNotFound(String),
}

pub type LocalResult<T> = std::result::Result<T, LocalError>;
