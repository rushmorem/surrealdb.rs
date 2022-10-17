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
        todo!()
    }

    async fn close(&mut self) -> Result<()> {
        todo!()
    }
}

pub struct Http<A>(pub A);

impl From<Http<SocketAddr>> for Http<ServerAddrs> {
    fn from(Http(address): Http<SocketAddr>) -> Self {
        Self(ServerAddrs {
            endpoint: address.to_string(),
            #[cfg(feature = "tls")]
            tls_config: None,
        })
    }
}

impl From<Http<&str>> for Http<ServerAddrs> {
    fn from(Http(address): Http<&str>) -> Self {
        Self(ServerAddrs {
            endpoint: address.to_owned(),
            #[cfg(feature = "tls")]
            tls_config: None,
        })
    }
}

impl From<Http<String>> for Http<ServerAddrs> {
    fn from(Http(endpoint): Http<String>) -> Self {
        Self(ServerAddrs {
            endpoint,
            #[cfg(feature = "tls")]
            tls_config: None,
        })
    }
}

impl<A> IntoFuture for Connect<Http<A>, Client>
where
    Http<A>: Into<Http<ServerAddrs>>,
{
    type Output = Result<Surreal<Client>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Surreal<Client>>>>>;

    fn into_future(self) -> Self::IntoFuture {
        let Http(mut address) = self.address.into();
        address.endpoint = format!("http://{}", address.endpoint);
        Box::pin(async move {
            let conn = Client::connect(address).await?;
            Ok(Surreal { conn: Some(conn) })
        })
    }
}

#[cfg(feature = "tls")]
pub struct Https<A>(pub A);

#[cfg(feature = "tls")]
impl From<Https<SocketAddr>> for Https<ServerAddrs> {
    fn from(Https(address): Https<SocketAddr>) -> Self {
        Self(ServerAddrs {
            endpoint: address.to_string(),
            tls_config: None,
        })
    }
}

#[cfg(feature = "tls")]
impl From<Https<&str>> for Https<ServerAddrs> {
    fn from(Https(address): Https<&str>) -> Self {
        Self(ServerAddrs {
            endpoint: address.to_owned(),
            tls_config: None,
        })
    }
}

#[cfg(feature = "tls")]
impl From<Https<String>> for Https<ServerAddrs> {
    fn from(Https(endpoint): Https<String>) -> Self {
        Self(ServerAddrs {
            endpoint,
            tls_config: None,
        })
    }
}

#[cfg(feature = "tls")]
impl<A> IntoFuture for Connect<Https<A>, Client>
where
    Https<A>: Into<Https<ServerAddrs>>,
{
    type Output = Result<Surreal<Client>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Surreal<Client>>>>>;

    fn into_future(self) -> Self::IntoFuture {
        let Https(mut address) = self.address.into();
        address.endpoint = format!("https://{}", address.endpoint);
        Box::pin(async move {
            let conn = Client::connect(address).await?;
            Ok(Surreal { conn: Some(conn) })
        })
    }
}
