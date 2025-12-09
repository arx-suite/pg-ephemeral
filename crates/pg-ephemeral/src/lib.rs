#![allow(dead_code)]

#[cfg(all(not(feature = "local"), not(feature = "containerized")))]
compile_error!("No backend selected. Enable at least one feature: `local` or `containerized`.");

#[cfg(feature = "local")]
compile_error!(
    "The `local` backend is still under development. For now, enable only the `containerized` feature."
);

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
