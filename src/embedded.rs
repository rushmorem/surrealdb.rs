//! Embedded database instance
//!
//! `SurrealDB` itself can be embedded in this library, allowing you to query it using the same
//! crate and API that you would use when connecting to it remotely via WebSockets or HTTP.
//! All [storage engines](crate::storage) are supported but you have to activate their feature
//! flags first.
//!
//! **NB**: Some storage engines like `TiKV` and `RocksDB` depend on non-Rust libraries so you need
//! to install those libraries before you can build this crate when you activate their feature
//! flags. Please refer to [these instructions](https://github.com/surrealdb/surrealdb/blob/main/doc/BUILDING.md)
//! for more details on how to install them. If you are on Linux and you use
//! [the Nix package manager](https://github.com/surrealdb/surrealdb/tree/main/pkg/nix#installing-nix)
//! you can just run
//!
//! ```bash
//! nix develop github:surrealdb/surrealdb
//! ```
//!
//! which will drop you into a shell with all the dependencies available. One tip you may find
//! useful is to only enable the in-memory engine (`mem`) during development. Besides letting you not
//! worry about those dependencies on your dev machine, it allows you to keep compile times low
//! during development while allowing you to test your code fully.
//!
//! # Examples
//!
//! Instantiating a global instance
//!
//! ```
//! use surrealdb_rs::{Result, Surreal};
//! use surrealdb_rs::embedded::Db;
//! use surrealdb_rs::storage::Mem;
//! use surrealdb_rs::StaticClient;
//!
//! static DB: Surreal<Db> = Surreal::new();
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     DB.connect::<Mem>(()).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! Instantiating an in-memory instance
//!
//! ```
//! use surrealdb_rs::Surreal;
//! use surrealdb_rs::storage::Mem;
//!
//! # #[tokio::main]
//! # async fn main() -> surrealdb_rs::Result<()> {
//! let db = Surreal::connect::<Mem>(()).await?;
//! # Ok(())
//! # }
//! ```
//!
//! Instantiating an in-memory strict instance
//!
//! ```
//! use surrealdb_rs::param::Strict;
//! use surrealdb_rs::Surreal;
//! use surrealdb_rs::storage::Mem;
//!
//! # #[tokio::main]
//! # async fn main() -> surrealdb_rs::Result<()> {
//! let db = Surreal::connect::<Mem>(Strict).await?;
//! # Ok(())
//! # }
//! ```
//!
//! Instantiating a file-backed instance
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() -> surrealdb_rs::Result<()> {
//! use surrealdb_rs::param::Strict;
//! use surrealdb_rs::Surreal;
//! use surrealdb_rs::storage::File;
//!
//! let db = Surreal::connect::<File>("temp.db").await?;
//! # Ok(())
//! # }
//! ```
//!
//! Instantiating a file-backed strict instance
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() -> surrealdb_rs::Result<()> {
//! use surrealdb_rs::param::Strict;
//! use surrealdb_rs::Surreal;
//! use surrealdb_rs::storage::File;
//!
//! let db = Surreal::connect::<File>(("temp.db", Strict)).await?;
//! # Ok(())
//! # }
//! ```
//!
//! Instantiating a TiKV instance
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() -> surrealdb_rs::Result<()> {
//! use surrealdb_rs::Surreal;
//! use surrealdb_rs::storage::TiKv;
//!
//! let db = Surreal::connect::<TiKv>("127.0.0.1:2379").await?;
//! # Ok(())
//! # }
//! ```
//!
//! Instantiating a TiKV strict instance
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() -> surrealdb_rs::Result<()> {
//! use surrealdb_rs::param::Strict;
//! use surrealdb_rs::Surreal;
//! use surrealdb_rs::storage::TiKv;
//!
//! let db = Surreal::connect::<TiKv>(("127.0.0.1:2379", Strict)).await?;
//! # Ok(())
//! # }
//! ```
//!
//! Authentication methods (`signup`, `signin`, `authentication` and `invalidate`) are not availabe
//! on `Db`
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() -> surrealdb_rs::Result<()> {
//! use serde::Deserialize;
//! use surrealdb_rs::Surreal;
//! use surrealdb_rs::storage::TiKv;
//!
//! const ACCOUNT: &str = "account";
//!
//! #[derive(Debug, Deserialize)]
//! struct Account {
//!    id: String,
//!    balance: String,
//! }
//!
//! let db = Surreal::connect::<TiKv>("127.0.0.1:2379").await?;
//!
//! db.use_ns("namespace").use_db("database").await?;
//!
//! let accounts: Vec<Account> = db.select(ACCOUNT).await?;
//!
//! dbg!(accounts);
//! # Ok(())
//! # }
//! ```

/// An embedded database
#[derive(Debug, Clone)]
pub struct Db {
    pub(crate) method: crate::method::Method,
}
