#![allow(dead_code)]

mod daemon;
mod error;
mod macros;
mod platform;

pub mod ephemeral;

pub use daemon::PgDaemon;
pub use ephemeral::{PgEphemeral, PgEphemeralBuilder};
pub use error::Error as EphemeralError;
pub use error::Result as EphemeralResult;
