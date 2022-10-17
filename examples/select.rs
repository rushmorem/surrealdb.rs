use serde::Deserialize;
use surrealdb_rs::param::Root;
use surrealdb_rs::protocol::Ws;
use surrealdb_rs::{Result, Surreal};

const USER: &str = "user";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct User {
    id: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut conn = Surreal::connect(Ws("localhost:8000")).await?;

    conn.signin(Root {
        user: "root",
        pass: "root",
    })
    .await?;

    conn.use_ns("namespace").use_db("database").await?;

    let _user: Option<User> = conn.select((USER, "doe")).await?;

    let _users: Vec<User> = conn.select(USER).await?;

    Ok(())
}
