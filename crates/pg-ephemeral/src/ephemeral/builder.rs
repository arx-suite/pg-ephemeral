use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tempfile::{Builder as TempDirBuilder, TempDir};

use super::constants::{DEFAULT_DB_NAME, DEFAULT_DB_USER, PROGRAM_POSTGRES, TMP_DIR_PREFIX};
use super::utils::random_free_port;
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

    pub fn build(self) -> EphemeralResult<PgEphemeral> {
        // database port
        let db_port = self.allocate_port()?;

        // database password
        let _ = self.db_password.check_valid()?;

        // data dir
        let temp_dir = self.temp_dir()?;

        // binary
        let bin_base_path = self.bin_base_path()?;

        Ok(PgEphemeral {
            db_user: self.db_user,
            db_pass: self.db_password,
            db_port,
            db_name: self.db_name,
            persist: self.persist_data_dir,
            dump_path: self.dump_path,
            temp_dir,
            bin_base_path,
        })
    }

    #[must_use]
    #[inline]
    fn allocate_port(&self) -> EphemeralResult<u16> {
        let db_port = match self.db_port {
            Some(db_port) => db_port,
            None => random_free_port(),
        };

        Ok(db_port)
    }

    #[must_use]
    #[inline]
    fn temp_dir(&self) -> EphemeralResult<TempDir> {
        let root_dir = TempDir::new()?;
        let pg_data_dir = TempDirBuilder::new()
            .disable_cleanup(!self.persist_data_dir)
            .prefix(TMP_DIR_PREFIX)
            .tempdir_in(root_dir)?;

        Ok(pg_data_dir)
    }

    #[must_use]
    fn bin_base_path(&self) -> EphemeralResult<PathBuf> {
        use crate::platform::{ProgramFinder, ProgramFinderImpl};

        let bin_base_path = match self.bin_base_path.clone() {
            Some(base_path) => {
                if !std::fs::exists(&base_path)? {
                    return Err(BuilderError::BinaryPathNotExists(base_path))?;
                }

                if !base_path.is_dir() {
                    return Err(BuilderError::BinaryPathNotFolder)?;
                }

                let postgres_bin_path = base_path.join(PROGRAM_POSTGRES);
                if !std::fs::exists(&postgres_bin_path)? {
                    return Err(BuilderError::BinaryNotFound {
                        binary: PROGRAM_POSTGRES.into(),
                        search_path: base_path,
                    })?;
                }

                Some(base_path)
            }
            None => None,
        };

        let bin_base_path = match bin_base_path {
            Some(bin_path) => bin_path,
            None => {
                let postgres_bin_path =
                    ProgramFinderImpl.find(PROGRAM_POSTGRES).ok_or_else(|| {
                        BuilderError::BinaryNotFound {
                            binary: PROGRAM_POSTGRES.into(),
                            search_path: "$PATH".into(),
                        }
                    })?;

                postgres_bin_path
                    .parent()
                    .ok_or_else(|| {
                        BuilderError::Custom(
                            "failed to get the directory of the postgres binary".into(),
                        )
                    })?
                    .to_path_buf()
            }
        };

        Ok(bin_base_path)
    }
}
