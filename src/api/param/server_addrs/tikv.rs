use crate::api::embedded::Db;
use crate::api::param::ServerAddrs;
use crate::api::param::Strict;
use crate::api::param::ToServerAddrs;
use crate::api::storage::TiKv;
use crate::api::Result;
use std::net::SocketAddr;
use url::Url;

impl ToServerAddrs<TiKv> for &str {
	type Client = Db;

	fn to_server_addrs(self) -> Result<ServerAddrs> {
		Ok(ServerAddrs {
			endpoint: Url::parse(&format!("tikv://{self}"))?,
			strict: false,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}

impl ToServerAddrs<TiKv> for SocketAddr {
	type Client = Db;

	fn to_server_addrs(self) -> Result<ServerAddrs> {
		Ok(ServerAddrs {
			endpoint: Url::parse(&format!("tikv://{self}"))?,
			strict: false,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}

impl ToServerAddrs<TiKv> for String {
	type Client = Db;

	fn to_server_addrs(self) -> Result<ServerAddrs> {
		Ok(ServerAddrs {
			endpoint: Url::parse(&format!("tikv://{self}"))?,
			strict: false,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}

impl<T> ToServerAddrs<TiKv> for (T, Strict)
where
	T: ToServerAddrs<TiKv>,
{
	type Client = Db;

	fn to_server_addrs(self) -> Result<ServerAddrs> {
		let mut address = self.0.to_server_addrs()?;
		address.strict = true;
		Ok(address)
	}
}
