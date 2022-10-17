use crate::param::ServerAddrs;
use crate::{Connect, Connection, Result, Surreal};
use async_trait::async_trait;
use std::future::{Future, IntoFuture};
use std::net::{IpAddr, SocketAddr};
use std::pin::Pin;
use surrealdb::sql::Value;

pub struct Client;

#[async_trait]
impl Connection for Client {
    type Output = Self;

    async fn connect(address: ServerAddrs) -> Result<Self::Output> {
        todo!()
    }

    async fn send(&mut self, msg: Value) -> Result<Option<Value>> {
        todo!()
    }

    async fn recv(&mut self) -> Result<Option<Value>> {
        Ok(None)
    }

    async fn close(&mut self) -> Result<()> {
        todo!()
    }
}

pub struct Ws<A>(pub A);

impl From<Ws<SocketAddr>> for Ws<ServerAddrs> {
    fn from(Ws(address): Ws<SocketAddr>) -> Self {
        Self(ServerAddrs {
            endpoint: address.to_string(),
            #[cfg(feature = "tls")]
            tls_config: None,
        })
    }
}

impl From<Ws<&str>> for Ws<ServerAddrs> {
    fn from(Ws(address): Ws<&str>) -> Self {
        Self(ServerAddrs {
            endpoint: address.to_owned(),
            #[cfg(feature = "tls")]
            tls_config: None,
        })
    }
}

impl From<Ws<String>> for Ws<ServerAddrs> {
    fn from(Ws(endpoint): Ws<String>) -> Self {
        Self(ServerAddrs {
            endpoint,
            #[cfg(feature = "tls")]
            tls_config: None,
        })
    }
}

impl<A> IntoFuture for Connect<Ws<A>, Client>
where
    Ws<A>: Into<Ws<ServerAddrs>>,
{
    type Output = Result<Surreal<Client>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Surreal<Client>>>>>;

    fn into_future(self) -> Self::IntoFuture {
        let Ws(mut address) = self.address.into();
        address.endpoint = format!("ws://{}", address.endpoint);
        Box::pin(async move {
            let conn = Client::connect(address).await?;
            Ok(Surreal { conn: Some(conn) })
        })
    }
}

#[cfg(feature = "tls")]
pub struct Wss<A>(pub A);

#[cfg(feature = "tls")]
impl From<Wss<SocketAddr>> for Wss<ServerAddrs> {
    fn from(Wss(address): Wss<SocketAddr>) -> Self {
        Self(ServerAddrs {
            endpoint: address.to_string(),
            tls_config: None,
        })
    }
}

#[cfg(feature = "tls")]
impl From<Wss<&str>> for Wss<ServerAddrs> {
    fn from(Wss(address): Wss<&str>) -> Self {
        Self(ServerAddrs {
            endpoint: address.to_owned(),
            tls_config: None,
        })
    }
}

#[cfg(feature = "tls")]
impl From<Wss<String>> for Wss<ServerAddrs> {
    fn from(Wss(endpoint): Wss<String>) -> Self {
        Self(ServerAddrs {
            endpoint,
            tls_config: None,
        })
    }
}

#[cfg(feature = "tls")]
impl<A> IntoFuture for Connect<Wss<A>, Client>
where
    Wss<A>: Into<Wss<ServerAddrs>>,
{
    type Output = Result<Surreal<Client>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Surreal<Client>>>>>;

    fn into_future(self) -> Self::IntoFuture {
        let Wss(mut address) = self.address.into();
        address.endpoint = format!("wss://{}", address.endpoint);
        Box::pin(async move {
            let conn = Client::connect(address).await?;
            Ok(Surreal { conn: Some(conn) })
        })
    }
}
