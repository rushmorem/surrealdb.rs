use serde::Deserialize;
use surrealdb_rs::param::Root;
use surrealdb_rs::ws::Ws;
use surrealdb_rs::{Result, Surreal};

#[derive(Debug, Deserialize)]
struct User {
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut conn = Surreal::connect::<Ws>("localhost:8000").await?;

    conn.signin(Root {
        user: "root",
        pass: "root",
    })
    .await?;

    conn.use_ns("namespace").use_db("database").await?;

    let user: Option<User> = conn.select("user:doe").await?;

    if let Some(user) = user {
        dbg!(user.name);
    }

    conn.close().await?;

    Ok(())
}
