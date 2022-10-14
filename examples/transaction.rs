use serde::{Deserialize, Serialize};
use surrealdb_rs::param::Root;
use surrealdb_rs::protocol::Ws;
use surrealdb_rs::{Result, Surreal};

const USER: &str = "user";

#[derive(Debug, Serialize, Deserialize)]
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

    let mut txn = conn.transaction().await?;

    let _user: User = txn
        .create(USER)
        .content(User {
            id: "john".to_owned(),
            name: "John Doe".to_owned(),
        })
        .await?;

    let _user: User = txn
        .create(USER)
        .content(User {
            id: "jane".to_owned(),
            name: "Jane Doe".to_owned(),
        })
        .await?;

    txn.commit().await?;

    Ok(())
}
