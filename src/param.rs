use crate::Result;
use serde::Serialize;
use surrealdb::sql::{self, Object, Table, Thing, Value};

pub struct ServerAddrs {
    pub(crate) endpoint: String,
}

impl From<&str> for ServerAddrs {
    fn from(address: &str) -> Self {
        Self {
            endpoint: address.to_owned(),
        }
    }
}

pub trait Credentials: Serialize {
    type Output;

    fn to_value(&self) -> Result<Value> {
        todo!()
    }
}

#[derive(Serialize)]
pub struct Root<'a> {
    pub user: &'a str,
    pub pass: &'a str,
}

impl<'a> Credentials for Root<'a> {
    type Output = ();
}

#[derive(Serialize)]
pub struct NameSpace<'a> {
    pub ns: &'a str,
    pub user: &'a str,
    pub pass: &'a str,
}

impl<'a> Credentials for NameSpace<'a> {
    type Output = String;
}

#[derive(Serialize)]
pub struct Database<'a> {
    pub ns: &'a str,
    pub db: &'a str,
    pub user: &'a str,
    pub pass: &'a str,
}

impl<'a> Credentials for Database<'a> {
    type Output = String;
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
    type Output = String;
}

pub enum Resource {
    Table(Table),
    RecordId(Thing),
}

impl<'a> From<&'a str> for Resource {
    fn from(resource: &'a str) -> Self {
        todo!()
    }
}

pub enum Query<'a> {
    Raw(&'a str),
    Typed(sql::Query),
}
