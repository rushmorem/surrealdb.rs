use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize)]
struct Params<'a> {
    table: &'a str,
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

    let _users: Vec<User> = conn
        .query("SELECT * FROM $table")
        .bind(Params { table: USER })
        .await?;

    Ok(())
}
