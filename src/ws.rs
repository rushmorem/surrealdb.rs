use crate::param::ServerAddrs;
use crate::{Connect, Connection, Result, Surreal};
use async_trait::async_trait;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;
use surrealdb::sql::Value;

pub struct Client;

#[async_trait]
impl Connection for Client {
    async fn connect(address: ServerAddrs) -> Result<Self> {
        todo!()
    }

    async fn send<M>(&mut self, msg: M) -> Result<Value>
    where
        M: Serialize + Send,
    {
        todo!()
    }

    async fn close(&mut self) -> Result<()> {
        todo!()
    }
}

pub struct Ws;

impl IntoFuture for Connect<Ws, Client> {
    type Output = Result<Surreal<Client>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Surreal<Client>>>>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            Ok(Surreal {
                conn: Client::connect(self.address).await?,
            })
        })
    }
}

pub struct Wss;
