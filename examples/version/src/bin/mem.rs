use surrealdb_rs::storage::Mem;
use surrealdb_rs::Surreal;

#[tokio::main]
async fn main() -> surrealdb_rs::Result<()> {
    tracing_subscriber::fmt::init();

    let client = Surreal::connect::<Mem>(()).await?;

    let version = client.version().await?;

    tracing::info!("{version:?}");

    Ok(())
}
