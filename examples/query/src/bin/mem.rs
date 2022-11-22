use serde::Deserialize;
use surrealdb_rs::param::from_value;
use surrealdb_rs::storage::Mem;
use surrealdb_rs::Surreal;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct User {
    id: String,
    name: String,
    company: String,
}

#[tokio::main]
async fn main() -> surrealdb_rs::Result<()> {
    tracing_subscriber::fmt::init();

    let db = Surreal::connect::<Mem>(()).await?;

    db.use_ns("namespace").use_db("database").await?;

    #[rustfmt::skip]
    let mut results = db
        .query("
            CREATE user
            SET name = $name,
                company = $company
        ")
        .bind("name", "John Doe")
        .bind("company", "ACME Corporation")
        .await?;

    let value = results.remove(0)?.remove(0);
    let user: User = from_value(&value)?;
    tracing::info!("{user:?}");

    Ok(())
}
