//! Parameter types

mod credentials;
mod jwt;
mod query;
mod resource;
mod server_addrs;

use crate::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
#[cfg(feature = "http")]
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
use surrealdb::sql;
use surrealdb::sql::Value;

pub use credentials::*;
pub use jwt::*;
pub use query::*;
pub use resource::*;
pub use server_addrs::*;

pub type RecordId = sql::Thing;

type UnitOp<'a> = InnerOp<'a, ()>;

#[derive(Debug, Serialize)]
#[serde(tag = "op", rename_all = "lowercase")]
enum InnerOp<'a, T> {
    Add { path: &'a str, value: T },
    Remove { path: &'a str },
    Replace { path: &'a str, value: T },
    Copy { from: &'a str, path: &'a str },
    Move { from: &'a str, path: &'a str },
    Test { path: &'a str, value: T },
}

#[derive(Debug)]
pub struct PatchOp(pub(crate) Value);

impl PatchOp {
    pub fn add<T>(path: &str, value: T) -> Self
    where
        T: Serialize,
    {
        let value = from_json(json!(InnerOp::Add { path, value }));
        Self(value)
    }

    pub fn remove(path: &str) -> Self {
        let value = from_json(json!(UnitOp::Remove { path }));
        Self(value)
    }

    pub fn replace<T>(path: &str, value: T) -> Self
    where
        T: Serialize,
    {
        let value = from_json(json!(InnerOp::Replace { path, value }));
        Self(value)
    }

    pub fn copy(from: &str, path: &str) -> Self {
        let value = from_json(json!(UnitOp::Copy { from, path }));
        Self(value)
    }

    pub fn mv(from: &str, path: &str) -> Self {
        let value = from_json(json!(UnitOp::Move { from, path }));
        Self(value)
    }

    pub fn test<T>(path: &str, value: T) -> Self
    where
        T: Serialize,
    {
        let value = from_json(json!(InnerOp::Test { path, value }));
        Self(value)
    }
}

#[derive(Debug)]
pub struct Param {
    pub(crate) query: Vec<sql::Value>,
    #[cfg(feature = "http")]
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) file: Option<PathBuf>,
}

impl Param {
    pub(crate) fn new(query: Vec<sql::Value>) -> Self {
        Self {
            query,
            #[cfg(feature = "http")]
            #[cfg(not(target_arch = "wasm32"))]
            file: None,
        }
    }

    #[cfg(feature = "http")]
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn file(file: PathBuf) -> Self {
        Self {
            query: Vec::new(),
            file: Some(file),
        }
    }
}

#[derive(Debug)]
pub enum DbResponse {
    Query(Vec<Result<Vec<sql::Value>>>),
    Other(sql::Value),
}

pub fn from_value<T>(value: &sql::Value) -> Result<T>
where
    T: DeserializeOwned,
{
    let bytes = serde_pack::to_vec(&value)?;
    let response = serde_pack::from_slice(&bytes)?;
    Ok(response)
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
