use testcontainers::TestcontainersError;

#[derive(Debug, thiserror::Error)]
pub enum ContainerizedError {
    #[error("testcontainer error: {0}")]
    TestContainerError(#[from] TestcontainersError),
}

pub type ContainerizedResult<T> = std::result::Result<T, ContainerizedError>;
