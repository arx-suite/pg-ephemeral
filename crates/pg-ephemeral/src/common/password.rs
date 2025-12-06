use std::path::PathBuf;

use super::constants::DEFAULT_DB_PASSWORD;
use crate::{EphemeralError, EphemeralResult};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum PasswordMethod {
    Text(String),
    #[cfg(feature = "cli")]
    Prompt,
    File {
        file_path: PathBuf,
    },
}

impl PasswordMethod {
    pub fn check_valid(&self) -> EphemeralResult<()> {
        use PasswordMethod::*;

        match self {
            File { file_path } => {
                if !std::fs::exists(file_path)
                    .map_err(|err| EphemeralError::PasswordMethodFailed(err.to_string()))?
                {
                    return Err(EphemeralError::PasswordMethodFailed(
                        "password file not found".into(),
                    ));
                }

                if !file_path.is_file() {
                    return Err(EphemeralError::PasswordMethodFailed(
                        "given path is not a file".into(),
                    ));
                }
            }
            _ => {}
        }

        Ok(())
    }
}

impl Default for PasswordMethod {
    fn default() -> Self {
        Self::Text(DEFAULT_DB_PASSWORD.into())
    }
}
