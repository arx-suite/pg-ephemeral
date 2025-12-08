use testcontainers::core::{WaitFor, wait::LogWaitStrategy};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};

use super::config::ContainerizedConfig;
use super::error::{ContainerizedError, ContainerizedResult};
use crate::Ephemeral;
use crate::common::constants::{
    CONTAINERIZED_ENV_DB, CONTAINERIZED_ENV_PASSWORD, CONTAINERIZED_ENV_USER,
};

pub struct Containerized {
    config: ContainerizedConfig,
    container: Option<ContainerAsync<GenericImage>>,
}

impl Containerized {
    pub fn new(config: ContainerizedConfig) -> Self {
        Self {
            config,
            container: None,
        }
    }
}

impl Ephemeral<ContainerizedError> for Containerized {
    async fn start(&mut self) -> ContainerizedResult<()> {
        let default_wait_strategy = WaitFor::log(LogWaitStrategy::stdout(
            "database system is ready to accept connections",
        ));

        let container =
            GenericImage::new(self.config.image_name.clone(), self.config.image_tag.into())
                .with_exposed_port(self.config.db_port.into())
                .with_wait_for(default_wait_strategy)
                .with_container_name(self.config.container_name.clone())
                .with_env_var(CONTAINERIZED_ENV_PASSWORD, self.config.db_pass.clone())
                .with_env_var(CONTAINERIZED_ENV_USER, self.config.db_user.clone())
                .with_env_var(CONTAINERIZED_ENV_DB, self.config.db_name.clone())
                .start()
                .await?;

        self.container = Some(container);

        Ok(())
    }

    async fn shutdown(&mut self) -> ContainerizedResult<()> {
        if let Some(ref container) = self.container {
            container.stop().await?
        }

        self.container = None;

        Ok(())
    }

    async fn is_running(&self) -> ContainerizedResult<bool> {
        if let Some(ref container) = self.container {
            Ok(container.is_running().await?)
        } else {
            Ok(false)
        }
    }
}
