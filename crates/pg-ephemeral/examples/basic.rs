#![allow(dead_code)]

use pg_ephemeral::{EphemeralResult, PgDaemon, log_info, log_trace};
use tokio::signal;

#[tokio::main]
async fn main() -> EphemeralResult<()> {
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let daemon = PgDaemon::with_default().await?;

    log_info!(
        "Ephemeral PostgreSQL server is running! Connect using: {}",
        daemon.db.connection_uri()
    );

    signal::ctrl_c().await?;

    log_trace!("program is shutdown");

    Ok(())
}
