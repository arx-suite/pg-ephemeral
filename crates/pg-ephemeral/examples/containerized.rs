use std::time::Duration;

use pg_ephemeral::Ephemeral;
use pg_ephemeral::containerized::{Containerized, ContainerizedConfig};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let config = ContainerizedConfig::new();
    let mut containerized = Containerized::new(config);

    containerized.start().await?;

    dbg!(containerized.is_running().await?);

    containerized.shutdown().await?;

    tokio::time::sleep(Duration::from_secs(5)).await;

    dbg!(containerized.is_running().await?);

    Ok(())
}
