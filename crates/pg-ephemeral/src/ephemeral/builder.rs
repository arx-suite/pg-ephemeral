use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use super::constants::{DEFAULT_DB_NAME, DEFAULT_DB_USER};
use super::{PasswordMethod, PgEphemeral};
use crate::EphemeralResult;

#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("binary directory does not exist: {0}")]
    BinaryPathNotExists(PathBuf),

    #[error("binary directory is not a folder")]
    BinaryPathNotFolder,

    #[error("could not locate the expected binary inside the binary directory")]
    BinaryNotFound {
        binary: String,
        search_path: PathBuf,
    },

    #[error("unable to allocate an available port for the database")]
    DatabasePortFailed,

    #[error("{0}")]
    Custom(String),
}

/// Builder for constructing an ephemeral PostgreSQL instance.
///
/// [`PgEphemeralBuilder`] configures how the temporary database environment
/// is created, initialized, and managed during the lifetime of the process.
#[derive(Default, Debug, Clone)]
pub struct PgEphemeralBuilder {
    /// Username for the temporary PostgreSQL instance.
    pub db_user: String,

    /// Method used to supply the database password.
    pub db_password: PasswordMethod,

    /// Port for the PostgreSQL server.
    /// If `None`, a random available port will be selected automatically.
    pub db_port: Option<u16>,

    /// Name of the default maintenance database created during initialization.
    pub db_name: String,

    /// If `true`, the data directory will be preserved after the
    /// [`PgEphemeral`] instance is dropped. Useful for inspection or debugging.
    pub persist_data_dir: bool,

    /// Path where the database should be dumped via `pg_dump` when the
    /// [`PgEphemeral`] instance is dropped.  
    /// If `None`, no dump will be performed.
    pub dump_path: Option<PathBuf>,

    /// Path from which the database should be restored via `psql` when the
    /// [`PgEphemeral`] instance starts.  
    /// If `None`, no data will be preloaded.
    pub load_path: Option<PathBuf>,

    /// Additional server configuration entries to place in `postgresql.conf`
    /// via `initdb -c key=value`. These correspond to PostgreSQL runtime
    /// settings (e.g. `shared_buffers`, `max_connections`, etc.).
    pub server_configs: HashMap<String, String>,

    /// Additional command-line arguments passed directly to the `initdb`
    /// binary (e.g., `--encoding=UTF8`).  
    /// These are **distinct** from PostgreSQL config settings (those passed with `-c`).
    pub initdb_args: HashMap<String, String>,

    /// Optional base directory containing the PostgreSQL binaries
    /// (`initdb`, `createdb`, `postgres`).  
    ///
    /// If `None`, binaries will be located using the system `$PATH`.  
    /// If provided, the search will check `bin_base_path` first, then fall
    /// back to `$PATH` to locate the required tools.
    pub bin_base_path: Option<PathBuf>,
}

impl PgEphemeralBuilder {
    pub fn new() -> Self {
        Self {
            db_user: DEFAULT_DB_USER.into(),
            db_name: DEFAULT_DB_NAME.into(),
            persist_data_dir: false,
            ..Default::default()
        }
    }

    pub fn from_connection_uri() -> Self {
        todo!()
    }

    #[inline]
    pub fn with_db_user(mut self, user: impl ToString) -> Self {
        self.db_user = user.to_string();
        self
    }

    #[inline]
    pub fn with_db_password(mut self, pass: PasswordMethod) -> Self {
        self.db_password = pass;
        self
    }

    #[inline]
    pub fn with_db_port(mut self, port: u16) -> Self {
        self.db_port = Some(port);
        self
    }

    #[inline]
    pub fn with_db_name(mut self, db_name: impl ToString) -> Self {
        self.db_name = db_name.to_string();
        self
    }

    #[must_use]
    #[inline]
    pub fn with_bin_base_path(mut self, path: impl AsRef<Path>) -> Self {
        self.bin_base_path = Some(PathBuf::from(path.as_ref()));
        self
    }

    #[inline]
    pub fn keep(mut self) -> Self {
        self.persist_data_dir = true;
        self
    }

    #[inline]
    pub fn with_config_param(mut self, key: &str, value: &str) -> Self {
        let _old = self.server_configs.insert(key.into(), value.into());
        #[cfg(feature = "tracing")]
        match _old {
            Some(old) => {
                tracing::warn!("overriding the server config param", %key, old_value = %old, new_value = %value);
            }
            _ => {}
        }
        self
    }

    #[inline]
    pub fn with_initdb_arg(mut self, key: &str, value: &str) -> Self {
        let _old = self.initdb_args.insert(key.into(), value.into());
        #[cfg(feature = "tracing")]
        match _old {
            Some(old) => {
                tracing::warn!("overriding the initdb arg", %key, old_value = %old, new_value = %value);
            }
            _ => {}
        }
        self
    }

    pub async fn build(self) -> EphemeralResult<PgEphemeral> {
        todo!()
    }
}
