use pg_ephemeral::{EphemeralResult, PgDaemon};

#[tokio::main]
async fn main() -> EphemeralResult<()> {
    let daemon = PgDaemon::with_default().await?;

    println!(
        "Ephemeral PostgreSQL server is running! Connect using: {}",
        daemon.db.connection_uri()
    );

    Ok(())
}
