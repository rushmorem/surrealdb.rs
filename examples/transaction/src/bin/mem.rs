use surrealdb::sql::statements::BeginStatement;
use surrealdb::sql::statements::CommitStatement;
use surrealdb_rs::storage::Mem;
use surrealdb_rs::Surreal;

#[tokio::main]
async fn main() -> surrealdb_rs::Result<()> {
	tracing_subscriber::fmt::init();

	let db = Surreal::connect::<Mem>(()).await?;

	db.use_ns("namespace").use_db("database").await?;

	#[rustfmt::skip]
    let response = db

        // Start transaction
        .query(BeginStatement)

        // Setup accounts
        .query("
            CREATE account:one SET balance = 135605.16;
            CREATE account:two SET balance = 91031.31;
        ")

        // Move money
        .query("
            UPDATE account:one SET balance += 300.00;
            UPDATE account:two SET balance -= 300.00;
        ")

        // Finalise
        .query(CommitStatement)
        .await?;

	for result in response {
		for value in result.into_inner()? {
			tracing::info!("{value}");
		}
	}

	Ok(())
}
