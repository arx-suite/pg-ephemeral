use std::{io, path::Path, process::Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};

use crate::ephemeral::constants::{PROGRAM_INITDB, PROGRAM_POSTGRES};
use crate::{EphemeralError, EphemeralResult, PgEphemeral, PgEphemeralBuilder};

#[derive(Debug)]
pub struct PgDaemon {
    pub db: PgEphemeral,
    pub process: Child,
}

impl PgDaemon {
    pub async fn with_default() -> EphemeralResult<Self> {
        let ephemeral = PgEphemeralBuilder::new().build()?;
        let process = Self::start_process(ephemeral.temp_dir.path()).await?;

        Ok(Self {
            db: ephemeral,
            process,
        })
    }

    pub async fn start(&self) -> EphemeralResult<()> {
        todo!()
    }

    pub async fn shutdown(self) -> EphemeralResult<()> {
        todo!()
    }

    async fn start_process(data_dir: &Path) -> EphemeralResult<Child> {
        Command::new(PROGRAM_INITDB)
            .arg(&data_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()
            .await?;

        let mut child = Command::new(PROGRAM_POSTGRES)
            .arg("-D")
            .arg(&data_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        Self::wait_ready(&mut child).await?;

        Ok(child)
    }

    async fn wait_ready(child: &mut Child) -> EphemeralResult<()> {
        let stdout = match child.stdout.take() {
            Some(stdout) => stdout,
            None => {
                return Err(EphemeralError::IOError(io::Error::other(
                    "failed to get std output",
                )));
            }
        };

        let mut reader = BufReader::new(stdout).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            println!("{line}");
        }

        let stderr = match child.stderr.take() {
            Some(stderr) => stderr,
            None => {
                return Err(EphemeralError::IOError(io::Error::other(
                    "failed to get std error",
                )));
            }
        };

        let mut reader = BufReader::new(stderr).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            println!("{line}");

            if line.contains("database system is ready") {
                break;
            }
        }

        Ok(())
    }
}
