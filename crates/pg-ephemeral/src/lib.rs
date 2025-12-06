#![allow(dead_code)]

pub(crate) mod common;
mod error;
mod local;
mod macros;
mod platform;

pub use error::Error as EphemeralError;
pub use error::Result as EphemeralResult;
