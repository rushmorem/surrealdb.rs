use crate::{Error, Result};
use serde::Serialize;
use std::borrow::Cow;
use std::net::{IpAddr, SocketAddr};
use surrealdb::sql::{self, Object, Thing, Value};

pub struct ServerAddrs {
    pub(crate) endpoint: String,
    #[cfg(feature = "tls")]
    pub(crate) tls_config: Option<()>,
}

pub trait Credentials {
    type Output;

    fn to_value(self) -> Result<Value>
    where
        Self: Sized;
}

#[derive(Serialize)]
pub struct Root<'a> {
    pub user: &'a str,
    pub pass: &'a str,
}

impl<'a> Credentials for Root<'a> {
    type Output = ();

    fn to_value(self) -> Result<Value> {
        todo!()
    }
}

#[derive(Serialize)]
pub struct NameSpace<'a> {
    pub ns: &'a str,
    pub user: &'a str,
    pub pass: &'a str,
}

impl<'a> Credentials for NameSpace<'a> {
    type Output = Jwt;

    fn to_value(self) -> Result<Value> {
        todo!()
    }
}

#[derive(Serialize)]
pub struct Database<'a> {
    pub ns: &'a str,
    pub db: &'a str,
    pub user: &'a str,
    pub pass: &'a str,
}

impl<'a> Credentials for Database<'a> {
    type Output = Jwt;

    fn to_value(self) -> Result<Value> {
        todo!()
    }
}

#[derive(Serialize)]
pub struct Scope<'a, P> {
    pub ns: &'a str,
    pub db: &'a str,
    pub sc: &'a str,
    #[serde(flatten)]
    pub params: P,
}

impl<'a, P> Credentials for Scope<'a, P>
where
    P: Serialize,
{
    type Output = Jwt;

    fn to_value(self) -> Result<Value> {
        todo!()
    }
}

pub trait Resource {
    type Output;

    fn to_value(self) -> Result<Value>
    where
        Self: Sized;
}

pub struct RecordId(Thing);

impl Resource for RecordId {
    type Output = crate::Record;

    fn to_value(self) -> Result<Value> {
        let RecordId(record_id) = self;
        Ok(record_id.into())
    }
}

impl<T: AsRef<str>> Resource for (&str, T) {
    type Output = crate::Record;

    fn to_value(self) -> Result<Value> {
        let (table, id) = self;
        let record_id = (table, id.as_ref()).into();
        RecordId(record_id).to_value()
    }
}

impl<T: AsRef<str>> Resource for (&String, T) {
    type Output = crate::Record;

    fn to_value(self) -> Result<Value> {
        let (table, id) = self;
        let record_id = (table.as_str(), id.as_ref()).into();
        RecordId(record_id).to_value()
    }
}

impl Resource for (String, String) {
    type Output = crate::Record;

    fn to_value(self) -> Result<Value> {
        let record_id = self.into();
        RecordId(record_id).to_value()
    }
}

pub struct Table(sql::Table);

impl Resource for Table {
    type Output = crate::Table;

    fn to_value(self) -> Result<Value> {
        let Table(table) = self;
        Ok(table.into())
    }
}

impl Resource for &str {
    type Output = crate::Table;

    fn to_value(self) -> Result<Value> {
        if self.contains(':') {
            // using the record IDs in strings is not supported
            return Err(Error);
        }
        let table = sql::Table(self.to_owned());
        Table(table).to_value()
    }
}

impl Resource for &String {
    type Output = crate::Table;

    fn to_value(self) -> Result<Value> {
        self.as_str().to_value()
    }
}

impl Resource for String {
    type Output = crate::Table;

    fn to_value(self) -> Result<Value> {
        if self.contains(':') {
            return Err(Error);
        }
        let table = sql::Table(self);
        Table(table).to_value()
    }
}

pub enum Query<'a> {
    Raw(Cow<'a, str>),
    Typed(sql::Query),
}

impl<'a> From<&'a str> for Query<'a> {
    fn from(qry: &'a str) -> Self {
        Self::Raw(Cow::Borrowed(qry))
    }
}

impl From<String> for Query<'static> {
    fn from(qry: String) -> Self {
        Self::Raw(Cow::Owned(qry))
    }
}

impl From<sql::Query> for Query<'static> {
    fn from(qry: sql::Query) -> Self {
        Self::Typed(qry)
    }
}

pub struct Jwt(pub(crate) String);
