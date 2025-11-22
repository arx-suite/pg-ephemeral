use std::net::{IpAddr, Ipv4Addr};

pub const DEFAULT_DB_USER: &'static str = "pg-temp-user";
pub const DEFAULT_DB_PASSWORD: &'static str = "pg-temp-password";
pub const DEFAULT_DB_NAME: &'static str = "pg-temp";
pub const DEFAULT_DB_HOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const DEFAULT_DB_PORT: u16 = 5433;

pub const TMP_DIR_PREFIX: &'static str = "pgtemp-";

pub const PROGRAM_INITDB: &'static str = "initdb";
pub const PROGRAM_POSTGRES: &'static str = "postgres";
