use std::result::Result;

/// main interface for interacting with the application
pub trait Ephemeral<E: std::error::Error> {
    fn start(&mut self) -> impl Future<Output = Result<(), E>>;
    fn shutdown(&mut self) -> impl Future<Output = Result<(), E>>;
    fn is_running(&self) -> impl Future<Output = Result<bool, E>>;
}
