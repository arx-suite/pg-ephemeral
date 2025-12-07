#![allow(dead_code)]

pub(crate) mod common;
mod error;
mod macros;

/// [Containerized] doesn't require any platform specific code
#[cfg(feature = "local")]
mod platform;

#[cfg(feature = "local")]
pub mod local;

#[cfg(feature = "containerized")]
pub mod containerized;

mod ephemeral;

pub use ephemeral::Ephemeral;

pub use error::Error as EphemeralError;
pub use error::Result as EphemeralResult;
