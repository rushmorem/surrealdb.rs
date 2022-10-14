#![allow(dead_code, unused_variables, unused_imports)]

mod authenticate;
mod begin;
mod change;
mod create;
mod delete;
mod err;
mod export;
mod health;
mod import;
mod info;
mod invalidate;
mod kill;
mod live;
mod modify;
mod query;
mod select;
mod set;
mod signin;
mod signup;
mod update;
mod use_db;
mod use_ns;
mod version;

#[cfg(feature = "http")]
pub mod http;
pub mod param;
#[cfg(feature = "ws")]
pub mod ws;

pub use authenticate::Authenticate;
pub use begin::Begin;
pub use change::Change;
pub use create::Create;
pub use delete::Delete;
pub use err::Error;
pub use export::Export;
pub use health::Health;
pub use import::Import;
pub use info::Info;
pub use invalidate::Invalidate;
pub use kill::Kill;
pub use live::Live;
pub use modify::Modify;
pub use query::Query;
pub use select::Select;
pub use set::Set;
pub use signin::Signin;
pub use signup::Signup;
pub use update::Update;
pub use use_db::UseDb;
pub use use_ns::UseNs;
pub use version::Version;

use async_trait::async_trait;
use serde::Serialize;
use std::marker::PhantomData;
use surrealdb::sql::{Object, Value};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
pub trait Connection {
    async fn connect(address: param::ServerAddrs) -> Result<Self>
    where
        Self: Sized;
    async fn send<M>(&mut self, msg: M) -> Result<Value>
    where
        M: Serialize + Send;
    async fn close(&mut self) -> Result<()>;
}

pub struct Surreal<C> {
    conn: C,
}

pub struct Connect<P, C> {
    address: param::ServerAddrs,
    protocol: PhantomData<P>,
    response: PhantomData<C>,
}

impl<C: Connection> Surreal<C> {
    pub fn connect<P>(address: impl Into<param::ServerAddrs>) -> Connect<P, C> {
        Connect {
            address: address.into(),
            protocol: PhantomData,
            response: PhantomData,
        }
    }

    pub fn info(&mut self) -> Info<C> {
        todo!()
    }

    pub fn use_ns(&mut self, ns: &str) -> UseNs<C> {
        todo!()
    }

    pub fn signup<R>(&mut self, credentials: impl param::Credentials<Output = R>) -> Signup<C, R> {
        todo!()
    }

    pub fn signin<R>(&mut self, credentials: impl param::Credentials<Output = R>) -> Signin<C, R> {
        todo!()
    }

    pub fn authenticate(&mut self, token: &str) -> Authenticate<C> {
        todo!()
    }

    pub fn invalidate(&mut self) -> Invalidate<C> {
        todo!()
    }

    pub fn kill(&mut self, query_id: Uuid) -> Kill<C> {
        todo!()
    }

    pub fn live(&mut self, table_name: &str) -> Live<C> {
        todo!()
    }

    pub fn set(&mut self, key: &str, value: impl Serialize) -> Set<C> {
        todo!()
    }

    pub fn query<'a>(&mut self, query: impl Into<param::Query<'a>>) -> Query<C> {
        todo!()
    }

    pub fn select<R>(&mut self, resource: impl Into<param::Resource>) -> Select<C, R> {
        todo!()
    }

    pub fn create(&mut self, resource: impl Into<param::Resource>) -> Create<C> {
        todo!()
    }

    pub fn update(&mut self, resource: impl Into<param::Resource>) -> Update<C> {
        todo!()
    }

    pub fn change(&mut self, resource: impl Into<param::Resource>) -> Change<C> {
        todo!()
    }

    pub fn modify(&mut self, resource: impl Into<param::Resource>) -> Modify<C> {
        todo!()
    }

    pub fn delete(&mut self, resource: impl Into<param::Resource>) -> Delete<C> {
        todo!()
    }

    pub fn begin(&mut self) -> Begin<C> {
        todo!()
    }

    pub fn import(&mut self) -> Import<C> {
        todo!()
    }

    pub fn version(&mut self) -> Version<C> {
        todo!()
    }

    pub fn health(&mut self) -> Health<C> {
        todo!()
    }

    pub async fn close(mut self) -> Result<()> {
        self.conn.close().await
    }
}

#[cfg(feature = "http")]
impl Surreal<http::Client> {
    pub fn export(&mut self) -> Export<http::Client> {
        todo!()
    }
}
