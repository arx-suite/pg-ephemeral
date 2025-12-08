use std::net::{IpAddr, Ipv4Addr};

pub const DEFAULT_DB_USER: &'static str = "pg-user";
pub const DEFAULT_DB_PASSWORD: &'static str = "pg-secret";
pub const DEFAULT_DB_NAME: &'static str = "pg-temp";
pub const DEFAULT_DB_HOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const DEFAULT_DB_PORT: u16 = 5433;

// [Containerized] related
#[cfg(feature = "containerized")]
mod containerized {
    use crate::containerized::PgImageTag;

    pub const CONTAINERIZED_IMAGE_NAME: &'static str = "postgres";
    pub const CONTAINERIZED_IMAGE_TAG: PgImageTag = PgImageTag::V175;
    pub const CONTAINERIZED_CONTAINER_NAME: &'static str = "pg-ephemeral";
    pub const CONTAINERIZED_ENV_PASSWORD: &'static str = "POSTGRES_PASSWORD";
    pub const CONTAINERIZED_ENV_USER: &'static str = "POSTGRES_USER";
    pub const CONTAINERIZED_ENV_DB: &'static str = "POSTGRES_DB";
}

#[cfg(feature = "containerized")]
pub use containerized::*;

// [Local] related
#[cfg(feature = "local")]
mod local {
    pub const LOCAL_PROGRAM_POSTGRES: &'static str = "postgres";
    pub const LOCAL_PROGRAM_INITDB: &'static str = "initdb";
    pub const LOCAL_TMP_DIR_PREFIX: &'static str = "pgtemp-";
}

#[cfg(feature = "local")]
pub use local::*;
