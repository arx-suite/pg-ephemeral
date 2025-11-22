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

    #[inline]
    pub fn connection_uri(&self) -> String {
        let uri = match self.db_pass.clone() {
            PasswordMethod::File { file_path: _ } => {
                format!(
                    "postgresql://{}@localhost:{}/{}",
                    self.db_user, self.db_port, self.db_name
                )
            }
            PasswordMethod::Text(pass) => {
                format!(
                    "postgresql://{}:{}@localhost:{}/{}",
                    self.db_user, pass, self.db_port, self.db_name
                )
            }
        };

        uri
    }
}
