use super::PgImageTag;

use crate::common::constants::{
    CONTAINERIZED_CONTAINER_NAME, CONTAINERIZED_IMAGE_NAME, CONTAINERIZED_IMAGE_TAG,
    DEFAULT_DB_NAME, DEFAULT_DB_PASSWORD, DEFAULT_DB_PORT, DEFAULT_DB_USER,
};

#[derive(Debug, Clone)]
pub struct ContainerizedConfig {
    pub db_user: String,
    pub db_pass: String,
    pub db_name: String,
    pub db_port: u16,
    pub image_name: String,
    pub image_tag: PgImageTag,
    pub container_name: String,
}

impl ContainerizedConfig {
    pub fn new() -> Self {
        Self {
            db_user: DEFAULT_DB_USER.into(),
            db_pass: DEFAULT_DB_PASSWORD.into(),
            db_name: DEFAULT_DB_NAME.into(),
            db_port: DEFAULT_DB_PORT,
            image_name: CONTAINERIZED_IMAGE_NAME.into(),
            image_tag: CONTAINERIZED_IMAGE_TAG,
            container_name: CONTAINERIZED_CONTAINER_NAME.into(),
        }
    }
}
