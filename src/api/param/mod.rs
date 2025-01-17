//! Parameter types

mod credentials;
mod jwt;
mod query;
mod resource;
mod server_addrs;

use crate::api::method::query_response::QueryResponse;
use crate::api::Result;
use dmp::Diff;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(any(
	feature = "protocol-http",
	feature = "kv-mem",
	feature = "kv-tikv",
	feature = "kv-rocksdb",
	feature = "kv-fdb",
))]
use std::path::PathBuf;
use surrealdb::sql;
use surrealdb::sql::Thing;
use surrealdb::sql::Value;

pub use credentials::*;
pub use jwt::*;
pub use query::*;
pub use resource::*;
pub use server_addrs::*;

/// Record ID
pub type RecordId = Thing;

type UnitOp<'a> = InnerOp<'a, ()>;

#[derive(Debug, Serialize)]
#[serde(tag = "op", rename_all = "lowercase")]
enum InnerOp<'a, T> {
	Add {
		path: &'a str,
		value: T,
	},
	Remove {
		path: &'a str,
	},
	Replace {
		path: &'a str,
		value: T,
	},
	Change {
		path: &'a str,
		value: String,
	},
}

/// A [JSON Patch] operation
///
/// From the official website:
///
/// > JSON Patch is a format for describing changes to a JSON document.
/// > It can be used to avoid sending a whole document when only a part has changed.
///
/// [JSON Patch]: https://jsonpatch.com/
#[derive(Debug)]
pub struct PatchOp(pub(crate) Value);

impl PatchOp {
	/// Adds a value to an object or inserts it into an array.
	///
	/// In the case of an array, the value is inserted before the given index.
	/// The `-` character can be used instead of an index to insert at the end of an array.
	///
	/// # Examples
	///
	/// ```
	/// # use serde_json::json;
	/// # use surrealdb_rs::param::PatchOp;
	/// PatchOp::add("/biscuits/1", json!({ "name": "Ginger Nut" }))
	/// # ;
	/// ```
	#[must_use]
	pub fn add<T>(path: &str, value: T) -> Self
	where
		T: Serialize,
	{
		let value = from_json(json!(InnerOp::Add {
			path,
			value
		}));
		Self(value)
	}

	/// Removes a value from an object or array.
	///
	/// # Examples
	///
	/// ```
	/// # use surrealdb_rs::param::PatchOp;
	/// PatchOp::remove("/biscuits")
	/// # ;
	/// ```
	///
	/// Remove the first element of the array at `biscuits`
	/// (or just removes the “0” key if `biscuits` is an object)
	///
	/// ```
	/// # use surrealdb_rs::param::PatchOp;
	/// PatchOp::remove("/biscuits/0")
	/// # ;
	/// ```
	#[must_use]
	pub fn remove(path: &str) -> Self {
		let value = from_json(json!(UnitOp::Remove {
			path
		}));
		Self(value)
	}

	/// Replaces a value.
	///
	/// Equivalent to a “remove” followed by an “add”.
	///
	/// # Examples
	///
	/// ```
	/// # use surrealdb_rs::param::PatchOp;
	/// PatchOp::replace("/biscuits/0/name", "Chocolate Digestive")
	/// # ;
	/// ```
	#[must_use]
	pub fn replace<T>(path: &str, value: T) -> Self
	where
		T: Serialize,
	{
		let value = from_json(json!(InnerOp::Replace {
			path,
			value
		}));
		Self(value)
	}

	/// Changes a value
	#[must_use]
	pub fn change(path: &str, diff: Diff) -> Self {
		let value = from_json(json!(UnitOp::Change {
			path,
			value: diff.text,
		}));
		Self(value)
	}
}

/// Holds the parameters given to the caller
#[derive(Debug)]
#[allow(dead_code)] // used by the embedded and remote connections
pub struct Param {
	pub(crate) query: Option<(sql::Query, BTreeMap<String, Value>)>,
	pub(crate) other: Vec<Value>,
	#[cfg(not(target_arch = "wasm32"))]
	#[cfg(any(
		feature = "protocol-http",
		feature = "kv-mem",
		feature = "kv-tikv",
		feature = "kv-rocksdb",
		feature = "kv-fdb",
	))]
	pub(crate) file: Option<PathBuf>,
}

impl Param {
	pub(crate) fn new(other: Vec<Value>) -> Self {
		Self {
			other,
			query: None,
			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(any(
				feature = "protocol-http",
				feature = "kv-mem",
				feature = "kv-tikv",
				feature = "kv-rocksdb",
				feature = "kv-fdb",
			))]
			file: None,
		}
	}

	pub(crate) fn query(query: sql::Query, bindings: BTreeMap<String, Value>) -> Self {
		Self {
			query: Some((query, bindings)),
			other: Vec::new(),
			#[cfg(not(target_arch = "wasm32"))]
			#[cfg(any(
				feature = "protocol-http",
				feature = "kv-mem",
				feature = "kv-tikv",
				feature = "kv-rocksdb",
				feature = "kv-fdb",
			))]
			file: None,
		}
	}

	#[cfg(not(target_arch = "wasm32"))]
	#[cfg(any(
		feature = "protocol-http",
		feature = "kv-mem",
		feature = "kv-tikv",
		feature = "kv-rocksdb",
		feature = "kv-fdb",
	))]
	pub(crate) fn file(file: PathBuf) -> Self {
		Self {
			query: None,
			other: Vec::new(),
			file: Some(file),
		}
	}
}

/// The database response sent from the router to the caller
#[derive(Debug)]
pub enum DbResponse {
	/// The response sent for the `query` method
	Query(QueryResponse),
	/// The response sent for any method except `query`
	Other(sql::Value),
}

/// Internal function that accepts anything serializable, be it a value, a slice,
/// or a string; and deserializes it into the deduced `<T>`
pub(crate) fn from_serializable<S, T>(thing: &S) -> Result<T>
where
	T: DeserializeOwned,
	S: Serialize + ?Sized,
{
	let bytes = serde_pack::to_vec(&thing)?;
	let response = serde_pack::from_slice(&bytes)?;

	Ok(response)
}

/// Deserializes a value `T` from `SurrealDB` [`Value`]
pub fn from_value<T>(value: &sql::Value) -> Result<T>
where
	T: DeserializeOwned,
{
	from_serializable(value)
}

pub(crate) fn from_json(json: JsonValue) -> sql::Value {
	match json {
		JsonValue::Null => sql::Value::None,
		JsonValue::Bool(boolean) => boolean.into(),
		JsonValue::Number(number) => match (number.as_u64(), number.as_i64(), number.as_f64()) {
			(Some(number), _, _) => number.into(),
			(_, Some(number), _) => number.into(),
			(_, _, Some(number)) => number.into(),
			_ => unreachable!(),
		},
		JsonValue::String(string) => string.into(),
		JsonValue::Array(array) => array.into_iter().map(from_json).collect::<Vec<_>>().into(),
		JsonValue::Object(object) => object
			.into_iter()
			.map(|(key, value)| (key, from_json(value)))
			.collect::<BTreeMap<_, _>>()
			.into(),
	}
}
