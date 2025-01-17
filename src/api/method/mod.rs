//! SurrealDB method types

mod authenticate;
mod begin;
mod cancel;
mod commit;
mod content;
mod create;
mod delete;
#[cfg(all(
	any(
		feature = "protocol-http",
		feature = "kv-mem",
		feature = "kv-tikv",
		feature = "kv-rocksdb",
		feature = "kv-fdb",
	),
	not(target_arch = "wasm32")
))]
mod export;
mod health;
#[cfg(all(
	any(
		feature = "protocol-http",
		feature = "kv-mem",
		feature = "kv-tikv",
		feature = "kv-rocksdb",
		feature = "kv-fdb",
	),
	not(target_arch = "wasm32")
))]
mod import;
mod invalidate;
mod kill;
mod live;
mod merge;
mod patch;
mod query;
mod select;
mod set;
mod signin;
mod signup;
mod unset;
mod update;
mod use_ns;
mod version;

#[cfg(test)]
mod tests;

pub use authenticate::Authenticate;
#[doc(hidden)] // Not supported yet
pub use begin::Begin;
#[doc(hidden)] // Not supported yet
pub use begin::Transaction;
#[doc(hidden)] // Not supported yet
pub use cancel::Cancel;
#[doc(hidden)] // Not supported yet
pub use commit::Commit;
pub use content::Content;
pub use create::Create;
pub use delete::Delete;
#[cfg(all(
	any(
		feature = "protocol-http",
		feature = "kv-mem",
		feature = "kv-tikv",
		feature = "kv-rocksdb",
		feature = "kv-fdb",
	),
	not(target_arch = "wasm32")
))]
#[cfg_attr(
	docsrs,
	doc(cfg(all(
		any(
			feature = "protocol-http",
			feature = "kv-mem",
			feature = "kv-tikv",
			feature = "kv-rocksdb",
			feature = "kv-fdb",
		),
		not(target_arch = "wasm32")
	)))
)]
pub use export::Export;
pub use health::Health;
#[cfg(all(
	any(
		feature = "protocol-http",
		feature = "kv-mem",
		feature = "kv-tikv",
		feature = "kv-rocksdb",
		feature = "kv-fdb",
	),
	not(target_arch = "wasm32")
))]
#[cfg_attr(
	docsrs,
	doc(cfg(all(
		any(
			feature = "protocol-http",
			feature = "kv-mem",
			feature = "kv-tikv",
			feature = "kv-rocksdb",
			feature = "kv-fdb",
		),
		not(target_arch = "wasm32")
	)))
)]
pub use import::Import;
pub use invalidate::Invalidate;
#[doc(hidden)] // Not supported yet
pub use kill::Kill;
#[doc(hidden)] // Not supported yet
pub use live::Live;
pub use merge::Merge;
pub use patch::Patch;
pub use query::response as query_response;
pub use query::Query;
pub use select::Select;
pub use set::Set;
pub use signin::Signin;
pub use signup::Signup;
pub use unset::Unset;
pub use update::Update;
pub use use_ns::UseNs;
pub use use_ns::UseNsDb;
pub use version::Version;

#[cfg(any(
	feature = "kv-mem",
	feature = "kv-tikv",
	feature = "kv-rocksdb",
	feature = "kv-fdb",
	feature = "kv-indxdb"
))]
use crate::api::embedded::Db;
#[cfg(feature = "protocol-http")]
use crate::api::net::HttpClient;
#[cfg(feature = "protocol-ws")]
use crate::api::net::WsClient;
use crate::api::param;
use crate::api::param::from_json;
use crate::api::param::ToServerAddrs;
use crate::api::Connect;
use crate::api::Connection;
use crate::api::ExtractRouter;
use crate::api::StaticClient;
use crate::api::Surreal;
use once_cell::sync::OnceCell;
use serde::Serialize;
use serde_json::json;
use std::marker::PhantomData;
#[cfg(all(
	any(
		feature = "protocol-http",
		feature = "kv-mem",
		feature = "kv-tikv",
		feature = "kv-rocksdb",
		feature = "kv-fdb",
	),
	not(target_arch = "wasm32")
))]
use std::path::Path;
use surrealdb::sql::Uuid;

/// The query method
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Method {
	/// Sends an authentication token to the server
	Authenticate,
	/// Perfoms a merge update operation
	Merge,
	/// Creates a record in a table
	Create,
	/// Deletes a record from a table
	Delete,
	/// Exports a database
	Export,
	/// Checks the health of the server
	Health,
	/// Imports a database
	Import,
	/// Invalidates a session
	Invalidate,
	/// Kills a live query
	#[doc(hidden)] // Not supported yet
	Kill,
	/// Starts a live query
	#[doc(hidden)] // Not supported yet
	Live,
	/// Perfoms a patch update operation
	Patch,
	/// Sends a raw query to the database
	Query,
	/// Selects a record or records from a table
	Select,
	/// Sets a parameter on the connection
	Set,
	/// Signs into the server
	Signin,
	/// Signs up on the server
	Signup,
	/// Removes a parameter from a connection
	Unset,
	/// Perfoms an update operation
	Update,
	/// Selects a namespace and database to use
	Use,
	/// Queries the version of the server
	Version,
}

impl Method {
	#[allow(dead_code)] // used by `ws` and `http`
	pub(crate) fn as_str(&self) -> &str {
		match self {
			Method::Authenticate => "authenticate",
			Method::Create => "create",
			Method::Delete => "delete",
			Method::Export => "export",
			Method::Health => "health",
			Method::Import => "import",
			Method::Invalidate => "invalidate",
			Method::Kill => "kill",
			Method::Live => "live",
			Method::Merge => "merge",
			Method::Patch => "patch",
			Method::Query => "query",
			Method::Select => "select",
			Method::Set => "set",
			Method::Signin => "signin",
			Method::Signup => "signup",
			Method::Unset => "unset",
			Method::Update => "update",
			Method::Use => "use",
			Method::Version => "version",
		}
	}
}

impl<C> StaticClient<C> for Surreal<C>
where
	C: Connection,
{
	fn connect<P>(&self, address: impl ToServerAddrs<P, Client = C>) -> Connect<C, ()> {
		Connect {
			router: Some(&self.router),
			address: address.to_server_addrs(),
			capacity: 0,
			client: PhantomData,
			response_type: PhantomData,
		}
	}
}

impl<C> Surreal<C>
where
	C: Connection,
{
	/// Creates a new static instance of the client
	///
	/// The static singleton ensures that a single database instance is available across very large
	/// or complicated applications. With the singleton, only one connection to the database is
	/// instantiated, and the database connection does not have to be shared across components
	/// or controllers.
	///
	/// # Examples
	///
	/// ```no_run
	/// use serde::{Serialize, Deserialize};
	/// use std::borrow::Cow;
	/// use surrealdb_rs::{Result, Surreal, StaticClient};
	/// use surrealdb_rs::param::Root;
	/// use surrealdb_rs::protocol::Ws;
	/// use surrealdb_rs::net::WsClient;
	///
	/// // Creates a new static instance of the client
	/// static DB: Surreal<WsClient> = Surreal::new();
	///
	/// #[derive(Serialize, Deserialize)]
	/// struct Person {
	///     name: Cow<'static, str>,
	/// }
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     // Connect to the database
	///     DB.connect::<Ws>("localhost:8000").await?;
	///
	///     // Log into the database
	///     DB.signin(Root {
	///         username: "root",
	///         password: "root",
	///     }).await?;
	///
	///     // Select a namespace + database
	///     DB.use_ns("test").use_db("test").await?;
	///
	///     // Create or update a specific record
	///     let _tobie: Option<Person> = DB.update(("person", "tobie"))
	///         .content(Person {
	///             name: "Tobie".into(),
	///         }).await?;
	///
	///     Ok(())
	/// }
	/// ```
	pub const fn new() -> Self {
		Self {
			router: OnceCell::new(),
		}
	}

	/// Connects to a local or remote database endpoint
	///
	/// # Examples
	///
	/// ```no_run
	/// use surrealdb_rs::Surreal;
	/// use surrealdb_rs::protocol::{Ws, Wss};
	///
	/// # #[tokio::main]
	/// # async fn main() -> surrealdb_rs::Result<()> {
	/// // Connect to a local endpoint
	/// let db = Surreal::connect::<Ws>("localhost:8000").await?;
	///
	/// // Connect to a remote endpoint
	/// let db = Surreal::connect::<Wss>("cloud.surrealdb.com").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub fn connect<P>(address: impl ToServerAddrs<P, Client = C>) -> Connect<'static, C, Self> {
		Connect {
			router: None,
			address: address.to_server_addrs(),
			capacity: 0,
			client: PhantomData,
			response_type: PhantomData,
		}
	}

	#[doc(hidden)] // Not supported yet
	pub fn transaction(self) -> Begin<C> {
		Begin {
			client: self,
		}
	}

	/// Switch to a specific namespace
	///
	/// # Examples
	///
	/// ```no_run
	/// # use surrealdb_rs::{Result, Surreal};
	/// # use surrealdb_rs::net::WsClient;
	/// # #[tokio::main]
	/// # async fn main() -> Result<()> {
	/// # let db = Surreal::<WsClient>::new();
	/// db.use_ns("test").use_db("test").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub fn use_ns(&self, ns: impl Into<String>) -> UseNs<C> {
		UseNs {
			router: self.router.extract(),
			ns: ns.into(),
		}
	}

	/// Assigns a value as a parameter for this connection
	///
	/// # Examples
	///
	/// ```no_run
	/// # use surrealdb_rs::{Result, Surreal};
	/// # use surrealdb_rs::net::WsClient;
	/// # use serde::Serialize;
	/// #[derive(Serialize)]
	/// struct Name<'a> {
	///     first: &'a str,
	///     last: &'a str,
	/// }
	///
	/// # #[tokio::main]
	/// # async fn main() -> Result<()> {
	/// # let db = Surreal::<WsClient>::new();
	/// // Assign the variable on the connection
	/// db.set("name", Name {
	///     first: "Tobie",
	///     last: "Morgan Hitchcock",
	/// }).await?;
	/// // Use the variable in a subsequent query
	/// db.query("CREATE person SET name = $name").await?;
	/// // Use the variable in a subsequent query
	/// db.query("SELECT * FROM person WHERE name.first = $name.first").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub fn set(&self, key: impl Into<String>, value: impl Serialize) -> Set<C> {
		Set {
			router: self.router.extract(),
			key: key.into(),
			value: Ok(from_json(json!(value))),
		}
	}

	/// Removes a parameter from this connection
	///
	/// # Examples
	///
	/// ```no_run
	/// # use surrealdb_rs::{Result, Surreal};
	/// # use surrealdb_rs::net::WsClient;
	/// # use serde::Serialize;
	/// #[derive(Serialize)]
	/// struct Name<'a> {
	///     first: &'a str,
	///     last: &'a str,
	/// }
	///
	/// # #[tokio::main]
	/// # async fn main() -> Result<()> {
	/// # let db = Surreal::<WsClient>::new();
	/// // Assign the variable on the connection
	/// db.set("name", Name {
	///     first: "Tobie",
	///     last: "Morgan Hitchcock",
	/// }).await?;
	/// // Use the variable in a subsequent query
	/// db.query("CREATE person SET name = $name").await?;
	/// // Remove the variable from the connection
	/// db.unset("name").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub fn unset(&self, key: impl Into<String>) -> Unset<C> {
		Unset {
			router: self.router.extract(),
			key: key.into(),
		}
	}

	/// Runs a set of SurrealQL statements against the database
	///
	/// # Examples
	///
	/// ```no_run
	/// # use serde::Deserialize;
	/// # use surrealdb_rs::{Result, Surreal};
	/// # use surrealdb_rs::net::WsClient;
	/// # use surrealdb_rs::param::from_value;
	/// # #[derive(Deserialize)]
	/// # struct Person;
	/// # #[tokio::main]
	/// # async fn main() -> Result<()> {
	/// # let db = Surreal::<WsClient>::new();
	/// // Run queries
	/// let result = db
	///     .query("CREATE person")
	///     .query("SELECT * FROM type::table($tb)")
	///     .bind(("tb", "person"))
	///     .await?;
	/// // Get the first result from the first query
	/// let created: Option<Person> = result.get(0, 0)?;
	/// // Get all of the results from the second query
	/// let people: Vec<Person> = result.get(1, ..)?;
	/// # Ok(())
	/// # }
	/// ```
	pub fn query(&self, query: impl param::Query) -> Query<C> {
		Query {
			router: self.router.extract(),
			query: vec![query.try_into_query()],
			bindings: Ok(Default::default()),
		}
	}

	/// Selects all records in a table, or a specific record
	///
	/// # Examples
	///
	/// ```no_run
	/// # use serde::Deserialize;
	/// # use surrealdb_rs::{Result, Surreal};
	/// # use surrealdb_rs::net::WsClient;
	/// # #[derive(Deserialize)]
	/// # struct Person;
	/// # #[tokio::main]
	/// # async fn main() -> Result<()> {
	/// # let db = Surreal::<WsClient>::new();
	/// // Select all records from a table
	/// let people: Vec<Person> = db.select("person").await?;
	/// // Select a specific record from a table
	/// let person: Option<Person> = db.select(("person", "h5wxrf2ewk8xjxosxtyc")).await?;
	/// // You can skip an unnecessary option if you know the record already exists
	/// let person: Person = db.select(("person", "h5wxrf2ewk8xjxosxtyc")).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub fn select<R>(&self, resource: impl param::Resource<R>) -> Select<C, R> {
		Select {
			router: self.router.extract(),
			resource: resource.into_db_resource(),
			range: None,
			response_type: PhantomData,
		}
	}

	/// Creates a record in the database
	///
	/// # Examples
	///
	/// ```no_run
	/// # use serde::{Serialize, Deserialize};
	/// # use surrealdb_rs::{Result, Surreal};
	/// # use surrealdb_rs::net::WsClient;
	/// # #[derive(Deserialize)]
	/// # struct Person;
	/// # #[derive(Serialize)]
	/// # struct Settings {
	/// #     active: bool,
	/// #     marketing: bool,
	/// # }
	/// # #[derive(Serialize)]
	/// # struct User<'a> {
	/// #     name: &'a str,
	/// #     settings: Settings,
	/// # }
	/// # #[tokio::main]
	/// # async fn main() -> Result<()> {
	/// # let db = Surreal::<WsClient>::new();
	/// // Create a record with a random ID
	/// let person: Person = db.create("person").await?;
	/// // Create a record with a specific ID
	/// let record: Person = db.create(("person", "tobie"))
	///     .content(User {
	///         name: "Tobie",
	///         settings: Settings {
	///             active: true,
	///             marketing: true,
	///         },
	///     })
	///     .await?;
	/// # Ok(())
	/// # }
	/// ```
	pub fn create<R>(&self, resource: impl param::Resource<R>) -> Create<C, R> {
		Create {
			router: self.router.extract(),
			resource: resource.into_db_resource(),
			response_type: PhantomData,
		}
	}

	/// Updates all records in a table, or a specific record
	///
	/// # Examples
	///
	/// Replace the current document / record data with the specified data.
	///
	/// ```no_run
	/// # use serde::{Serialize, Deserialize};
	/// # use surrealdb_rs::{Result, Surreal};
	/// # use surrealdb_rs::net::WsClient;
	/// # #[derive(Deserialize)]
	/// # struct Person;
	/// # #[derive(Serialize)]
	/// # struct Settings {
	/// #     active: bool,
	/// #     marketing: bool,
	/// # }
	/// # #[derive(Serialize)]
	/// # struct User<'a> {
	/// #     name: &'a str,
	/// #     settings: Settings,
	/// # }
	/// # #[tokio::main]
	/// # async fn main() -> Result<()> {
	/// # let db = Surreal::<WsClient>::new();
	/// // Update all records in a table
	/// let people: Vec<Person> = db.update("person").await?;
	/// // Update a record with a specific ID
	/// let person: Option<Person> = db.update(("person", "tobie"))
	///     .content(User {
	///         name: "Tobie",
	///         settings: Settings {
	///             active: true,
	///             marketing: true,
	///         },
	///     })
	///     .await?;
	/// # Ok(())
	/// # }
	/// ```
	///
	/// Merge the current document / record data with the specified data.
	///
	/// ```no_run
	/// # use serde::{Serialize, Deserialize};
	/// # use surrealdb_rs::{Result, Surreal};
	/// # use surrealdb_rs::net::WsClient;
	/// # use time::OffsetDateTime;
	/// # #[derive(Deserialize)]
	/// # struct Person;
	/// # #[derive(Serialize)]
	/// # struct UpdatedAt {
	/// #     updated_at: OffsetDateTime,
	/// # }
	/// # #[derive(Serialize)]
	/// # struct Settings {
	/// #     active: bool,
	/// # }
	/// # #[derive(Serialize)]
	/// # struct User {
	/// #     updated_at: OffsetDateTime,
	/// #     settings: Settings,
	/// # }
	/// # #[tokio::main]
	/// # async fn main() -> Result<()> {
	/// # let db = Surreal::<WsClient>::new();
	/// // Update all records in a table
	/// let people: Vec<Person> = db.update("person")
	///     .merge(UpdatedAt {
	///         updated_at: OffsetDateTime::now_utc(),
	///     })
	///     .await?;
	/// // Update a record with a specific ID
	/// let person: Option<Person> = db.update(("person", "tobie"))
	///     .merge(User {
	///         updated_at: OffsetDateTime::now_utc(),
	///         settings: Settings {
	///             active: true,
	///         },
	///     })
	///     .await?;
	/// # Ok(())
	/// # }
	/// ```
	///
	/// Apply [JSON Patch](https://jsonpatch.com) changes to all records, or a specific record, in the database.
	///
	/// ```no_run
	/// # use serde::{Serialize, Deserialize};
	/// # use surrealdb_rs::{Result, Surreal};
	/// # use surrealdb_rs::net::WsClient;
	/// # use surrealdb_rs::param::PatchOp;
	/// # use time::OffsetDateTime;
	/// # #[derive(Deserialize)]
	/// # struct Person;
	/// # #[derive(Serialize)]
	/// # struct UpdatedAt {
	/// #     updated_at: OffsetDateTime,
	/// # }
	/// # #[derive(Serialize)]
	/// # struct Settings {
	/// #     active: bool,
	/// # }
	/// # #[derive(Serialize)]
	/// # struct User {
	/// #     updated_at: OffsetDateTime,
	/// #     settings: Settings,
	/// # }
	/// # #[tokio::main]
	/// # async fn main() -> Result<()> {
	/// # let db = Surreal::<WsClient>::new();
	/// // Update all records in a table
	/// let people: Vec<Person> = db.update("person")
	///     .patch(PatchOp::replace("/created_at", OffsetDateTime::now_utc()))
	///     .await?;
	/// // Update a record with a specific ID
	/// let person: Option<Person> = db.update(("person", "tobie"))
	///     .patch(PatchOp::replace("/settings/active", false))
	///     .patch(PatchOp::add("/tags", ["developer", "engineer"]))
	///     .patch(PatchOp::remove("/temp"))
	///     .await?;
	/// # Ok(())
	/// # }
	/// ```
	pub fn update<R>(&self, resource: impl param::Resource<R>) -> Update<C, R> {
		Update {
			router: self.router.extract(),
			resource: resource.into_db_resource(),
			range: None,
			response_type: PhantomData,
		}
	}

	/// Deletes all records, or a specific record
	///
	/// # Examples
	///
	/// ```no_run
	/// # use surrealdb_rs::{Result, Surreal};
	/// # use surrealdb_rs::net::WsClient;
	/// # #[tokio::main]
	/// # async fn main() -> Result<()> {
	/// # let db = Surreal::<WsClient>::new();
	/// // Delete all records from a table
	/// db.delete("person").await?;
	/// // Delete a specific record from a table
	/// db.delete(("person", "h5wxrf2ewk8xjxosxtyc")).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub fn delete<R>(&self, resource: impl param::Resource<R>) -> Delete<C, R> {
		Delete {
			router: self.router.extract(),
			resource: resource.into_db_resource(),
			range: None,
			response_type: PhantomData,
		}
	}

	/// Returns the version of the server
	///
	/// # Examples
	///
	/// ```no_run
	/// # use surrealdb_rs::{Result, Surreal};
	/// # use surrealdb_rs::net::WsClient;
	/// # #[tokio::main]
	/// # async fn main() -> Result<()> {
	/// # let db = Surreal::<WsClient>::new();
	/// let version = db.version().await?;
	/// # Ok(())
	/// # }
	/// ```
	pub fn version(&self) -> Version<C> {
		Version {
			router: self.router.extract(),
		}
	}

	/// Checks whether the server is healthy or not
	///
	/// # Examples
	///
	/// ```no_run
	/// # use surrealdb_rs::{Result, Surreal};
	/// # use surrealdb_rs::net::WsClient;
	/// # #[tokio::main]
	/// # async fn main() -> Result<()> {
	/// # let db = Surreal::<WsClient>::new();
	/// db.health().await?;
	/// # Ok(())
	/// # }
	/// ```
	pub fn health(&self) -> Health<C> {
		Health {
			router: self.router.extract(),
		}
	}

	#[doc(hidden)] // Not supported yet
	pub fn kill(&self, query_id: Uuid) -> Kill<C> {
		Kill {
			router: self.router.extract(),
			query_id,
		}
	}

	#[doc(hidden)] // Not supported yet
	pub fn live(&self, table_name: impl Into<String>) -> Live<C> {
		Live {
			router: self.router.extract(),
			table_name: table_name.into(),
		}
	}
}

#[allow(unused_macros)] // used by `ws` and `http`
macro_rules! auth_methods {
	($client:ty) => {
		/// Signs this connection up to a specific authentication scope
		pub fn signup<R>(
			&self,
			credentials: impl param::Credentials<param::Signup, R>,
		) -> Signup<$client, R> {
			Signup {
				router: self.router.extract(),
				credentials: Ok(from_json(json!(credentials))),
				response_type: PhantomData,
			}
		}

		/// Signs this connection in to a specific authentication scope
		pub fn signin<R>(
			&self,
			credentials: impl param::Credentials<param::Signin, R>,
		) -> Signin<$client, R> {
			Signin {
				router: self.router.extract(),
				credentials: Ok(from_json(json!(credentials))),
				response_type: PhantomData,
			}
		}

		/// Invalidates the authentication for the current connection
		pub fn invalidate(&self) -> Invalidate<$client> {
			Invalidate {
				router: self.router.extract(),
			}
		}

		/// Authenticates the current connection with a JWT token
		pub fn authenticate(&self, token: impl Into<param::Jwt>) -> Authenticate<$client> {
			Authenticate {
				router: self.router.extract(),
				token: token.into(),
			}
		}
	};
}

#[allow(unused_macros)] // used by the embedded database and `http`
macro_rules! backup_methods {
	($client:ty) => {
		/// Dumps the database contents to a file
		#[cfg(not(target_arch = "wasm32"))]
		#[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))]
		pub fn export<P>(&self, file: P) -> Export<$client>
		where
			P: AsRef<Path>,
		{
			Export {
				router: self.router.extract(),
				file: file.as_ref().to_owned(),
			}
		}

		/// Restores the database from a file
		#[cfg(not(target_arch = "wasm32"))]
		#[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))]
		pub fn import<P>(&self, file: P) -> Import<$client>
		where
			P: AsRef<Path>,
		{
			Import {
				router: self.router.extract(),
				file: file.as_ref().to_owned(),
			}
		}
	};
}

#[cfg(feature = "protocol-ws")]
#[cfg_attr(docsrs, doc(cfg(feature = "protocol-ws")))]
impl Surreal<WsClient> {
	auth_methods!(WsClient);
}

#[cfg(feature = "protocol-http")]
#[cfg_attr(docsrs, doc(cfg(feature = "protocol-http")))]
impl Surreal<HttpClient> {
	auth_methods!(HttpClient);
	backup_methods!(HttpClient);
}

#[cfg(any(
	feature = "kv-mem",
	feature = "kv-tikv",
	feature = "kv-rocksdb",
	feature = "kv-fdb",
	feature = "kv-indxdb"
))]
#[cfg_attr(
	docsrs,
	doc(cfg(any(
		feature = "kv-mem",
		feature = "kv-tikv",
		feature = "kv-rocksdb",
		feature = "kv-fdb",
		feature = "kv-indxdb"
	)))
)]
impl Surreal<Db> {
	backup_methods!(Db);
}
