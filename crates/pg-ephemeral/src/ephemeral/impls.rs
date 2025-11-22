use std::path::PathBuf;
use tempfile::TempDir;

use super::{PasswordMethod, PgEphemeralBuilder};

#[derive(Debug)]
pub struct PgEphemeral {
    pub db_user: String,
    pub db_pass: PasswordMethod,
    pub db_port: u16,
    pub db_name: String,
    pub persist: bool,
    pub dump_path: Option<PathBuf>,
    pub temp_dir: TempDir,
    pub bin_base_path: PathBuf,
}

impl PgEphemeral {
    pub fn builder() -> PgEphemeralBuilder {
        PgEphemeralBuilder::new()
    }

    pub fn connection_uri(&self) -> String {
        todo!()
    }
}
