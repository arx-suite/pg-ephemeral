use crate::EphemeralResult;

/// main interface for interacting with the application
pub trait Ephemeral {
    fn start(&mut self) -> impl Future<Output = EphemeralResult<()>>;
    fn is_running(&self) -> impl Future<Output = EphemeralResult<bool>>;
    fn shutdown(&mut self) -> impl Future<Output = EphemeralResult<()>>;
}
