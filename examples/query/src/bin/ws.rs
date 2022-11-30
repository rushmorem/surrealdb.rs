use serde::Deserialize;
use serde::Serialize;
use surrealdb_rs::param::Root;
use surrealdb_rs::protocol::Ws;
use surrealdb_rs::Surreal;

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct User {
	id: String,
	name: String,
	company: String,
}

#[tokio::main]
async fn main() -> surrealdb_rs::Result<()> {
	tracing_subscriber::fmt::init();

	let db = Surreal::connect::<Ws>("localhost:8000").await?;

	db.signin(Root {
		username: "root",
		password: "root",
	})
	.await?;

	db.use_ns("namespace").use_db("database").await?;

	#[rustfmt::skip]
    let results = db
        .query("
            CREATE user
            SET name = $name,
                company = $company
        ")
		.bind(User {
			id: "john".to_owned(),
			name: "John Doe".to_owned(),
			company: "ACME Corporation".to_owned(),
		})
        .await?;

	// print the created user:
	let user: Option<User> = results.get(0, 0)?;
	tracing::info!("{user:?}");

	let response = db.query("SELECT * FROM user").await?;

	// print all users:
	let users: Vec<User> = response.get(0, ..)?;
	tracing::info!("{users:?}");

	Ok(())
}
