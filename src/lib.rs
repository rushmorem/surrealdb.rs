#![allow(dead_code, unused_variables, unused_imports)]

mod authenticate;
mod begin;
mod cancel;
mod change;
mod commit;
mod content;
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
mod patches;
mod query;
mod select;
mod set;
mod signin;
mod signup;
mod update;
mod use_db;
mod use_ns;
mod version;

#[cfg(any(feature = "http", feature = "ws"))]
pub mod net;
pub mod param;
#[cfg(any(feature = "http", feature = "ws"))]
pub mod protocol;

pub use authenticate::Authenticate;
pub use begin::{Begin, Transaction};
pub use cancel::Cancel;
pub use change::Change;
pub use commit::Commit;
pub use content::Content;
pub use create::Create;
pub use delete::Delete;
pub use err::{Error, ErrorKind};
pub use export::Export;
pub use health::Health;
pub use import::Import;
pub use info::Info;
pub use invalidate::Invalidate;
pub use kill::Kill;
pub use live::Live;
pub use modify::Modify;
pub use patches::Patches;
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
use std::mem;
use surrealdb::sql::{Object, Value};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Record;
pub struct Table;

#[async_trait]
pub trait Connection: Send + Sync + 'static {
    type Output;
    async fn connect(address: param::ServerAddrs) -> Result<Self::Output>;
    async fn send(&mut self, msg: Value) -> Result<Option<Value>>;
    async fn recv(&mut self) -> Result<Option<Value>>;
    async fn close(&mut self) -> Result<()>;
}

pub struct Connect<A, C> {
    address: A,
    response: PhantomData<C>,
}

pub struct Surreal<C: Connection> {
    conn: Option<C>,
}

impl<C: Connection> Surreal<C> {
    pub fn connect<A>(address: A) -> Connect<A, C> {
        Connect {
            address,
            response: PhantomData,
        }
    }

    pub fn transaction(self) -> Begin<C> {
        todo!()
    }

    pub fn info(&mut self) -> Info<C> {
        todo!()
    }

    pub fn use_ns(&mut self, ns: &str) -> UseNs<C> {
        todo!()
    }

    pub fn use_db(&mut self, db: &str) -> UseDb<C> {
        todo!()
    }

    pub fn signup<R>(&mut self, credentials: impl param::Credentials<Output = R>) -> Signup<C, R> {
        todo!()
    }

    pub fn signin<R>(&mut self, credentials: impl param::Credentials<Output = R>) -> Signin<C, R> {
        todo!()
    }

    pub fn authenticate(&mut self, token: param::Jwt) -> Authenticate<C> {
        todo!()
    }

    pub fn invalidate(&mut self) -> Invalidate<C> {
        todo!()
    }

    pub fn set(&mut self, key: &str, value: impl Serialize) -> Set<C> {
        todo!()
    }

    pub fn query<'a, R>(&mut self, query: impl Into<param::Query<'a>>) -> Query<C, R> {
        todo!()
    }

    pub fn select<T, R>(&mut self, resource: impl param::Resource<Output = T>) -> Select<C, T, R> {
        todo!()
    }

    pub fn create<R>(&mut self, resource: impl param::Resource) -> Create<C, R> {
        todo!()
    }

    pub fn update<T, R>(&mut self, resource: impl param::Resource<Output = T>) -> Update<C, T, R> {
        todo!()
    }

    pub fn change<T, R>(&mut self, resource: impl param::Resource<Output = T>) -> Change<C, T, R> {
        todo!()
    }

    pub fn modify<T, R>(&mut self, resource: impl param::Resource<Output = T>) -> Modify<C, T, R> {
        todo!()
    }

    pub fn delete(&mut self, resource: impl param::Resource) -> Delete<C> {
        todo!()
    }

    pub fn import<'a>(
        &mut self,
        ns: &str,
        db: &str,
        statements: impl Into<param::Query<'a>>,
    ) -> Import<C> {
        todo!()
    }

    pub async fn close(mut self) -> Result<()> {
        if let Some(conn) = &mut self.conn {
            conn.close().await?;
        }
        Ok(())
    }
}

#[doc(hidden)] // hide these for now until the server re-enables live queries
#[cfg(feature = "ws")]
impl Surreal<net::WsClient> {
    pub fn kill(&mut self, query_id: Uuid) -> Kill<net::WsClient> {
        todo!()
    }

    pub fn live(&mut self, table_name: &str) -> Live<net::WsClient> {
        todo!()
    }
}

#[cfg(feature = "http")]
impl Surreal<net::HttpClient> {
    pub fn version(&mut self) -> Version<net::HttpClient> {
        todo!()
    }

    pub fn health(&mut self) -> Health<net::HttpClient> {
        todo!()
    }

    pub fn export(&mut self, ns: &str, db: &str) -> Export<net::HttpClient> {
        todo!()
    }
}

impl<C: Connection> Drop for Surreal<C> {
    fn drop(&mut self) {
        let client = mem::replace(self, Surreal { conn: None });
        tokio::spawn(async move {
            if let Err(_error) = client.close().await {
                // TODO log the reason
                tracing::warn!("failed to close database connection");
            }
        });
    }
}
