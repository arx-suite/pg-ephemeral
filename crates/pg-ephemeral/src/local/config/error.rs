use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum LocalBuilderError {
    #[error("I/O operation failed: {0}")]
    IOError(#[from] std::io::Error),

    #[error("binary directory does not exist: {0}")]
    BinaryPathNotExists(PathBuf),

    #[error("binary directory is not a folder")]
    BinaryPathNotFolder,

    #[error("could not locate the expected binary inside the binary directory")]
    BinaryNotFound {
        binary: String,
        search_path: PathBuf,
    },

    #[error("unable to allocate an available port for the database")]
    DatabasePortFailed,

    #[error("{0}")]
    Custom(String),
}

pub type LocalBuilderResult<T> = std::result::Result<T, LocalBuilderError>;
