#![allow(dead_code, unused_imports)]

use std::io;

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use unix::Sys;
#[cfg(windows)]
pub use windows::Sys;

/// Abstraction for platform-specific system operations required to manage
/// child processes
pub trait SysT: SysInfo + Sized {
    /// Initialize and collect all system information needed to spawn a child
    fn new() -> io::Result<Self>;

    /// Spawn a new child process
    fn spawn();

    /// Kill a child process
    fn kill();
}

/// Provides system information about the current process and environment
pub trait SysInfo {
    /// Host machine name
    fn sysname(&self) -> String;

    /// Username of the current process owner
    fn user(&self) -> String;

    /// Whether the current user has root-level privileges
    fn has_root_privilege(&self) -> bool;
}
