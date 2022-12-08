#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, deny(warnings))]

//! This SurrealDB library enables simple and advanced querying of a remote or embedded database from
//! server-side or client-side code. All connections to SurrealDB are made over WebSockets by default (HTTP
//! and embedded databases are also supported), and automatically reconnect when the connection is terminated.
//!
//! # Examples
//!
//! ```no_run
//! use serde::{Serialize, Deserialize};
//! use serde_json::json;
//! use std::borrow::Cow;
//! use surrealdb_rs::{Result, Surreal};
//! use surrealdb_rs::param::Root;
//! use surrealdb_rs::protocol::Ws;
//! use ulid::Ulid;
//!
//! #[derive(Serialize, Deserialize)]
//! struct Name {
//!     first: Cow<'static, str>,
//!     last: Cow<'static, str>,
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct Person {
//!     title: Cow<'static, str>,
//!     name: Name,
//!     marketing: bool,
//!     identifier: Ulid,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let db = Surreal::connect::<Ws>("127.0.0.1:8000").await?;
//!
//!     // Signin as a namespace, database, or root user
//!     db.signin(Root {
//!         username: "root",
//!         password: "root",
//!     }).await?;
//!
//!     // Select a specific namespace / database
//!     db.use_ns("test").use_db("test").await?;
//!
//!     // Create a new person with a random ID
//!     let created: Person = db.create("person")
//!         .content(Person {
//!             title: "Founder & CEO".into(),
//!             name: Name {
//!                 first: "Tobie".into(),
//!                 last: "Morgan Hitchcock".into(),
//!             },
//!             marketing: true,
//!             identifier: Ulid::new(),
//!         })
//!         .await?;
//!
//!     // Create a new person with a specific ID
//!     let created: Person = db.create(("person", "jaime"))
//!         .content(Person {
//!             title: "Founder & COO".into(),
//!             name: Name {
//!                 first: "Jaime".into(),
//!                 last: "Morgan Hitchcock".into(),
//!             },
//!             marketing: false,
//!             identifier: Ulid::new(),
//!         })
//!         .await?;
//!
//!     // Update a person record with a specific ID
//!     let updated: Person = db.update(("person", "jaime"))
//!         .merge(json!({"marketing": true}))
//!         .await?;
//!
//!     // Select all people records
//!     let people: Vec<Person> = db.select("person").await?;
//!
//!     // Perform a custom advanced query
//!     let groups = db
//!         .query("SELECT marketing, count() FROM type::table($tb) GROUP BY marketing")
//!         .bind(("tb", "person"))
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

pub(crate) mod api;

pub use api::*;
