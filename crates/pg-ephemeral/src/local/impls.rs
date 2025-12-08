use crate::Ephemeral;
use crate::platform::sys::Sys;

use super::config::LocalConfig;
use super::error::{LocalError, LocalResult};

pub struct Local {
    config: LocalConfig,
    process: Sys,
}

impl Local {
    pub fn new() -> Self {
        todo!()
    }
}

impl Ephemeral<LocalError> for Local {
    async fn start(&mut self) -> LocalResult<()> {
        todo!()
    }

    async fn shutdown(&mut self) -> LocalResult<()> {
        todo!()
    }

    async fn is_running(&self) -> LocalResult<bool> {
        todo!()
    }
}
