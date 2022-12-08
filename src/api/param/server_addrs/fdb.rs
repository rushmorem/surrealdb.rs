use crate::api::embedded::Db;
use crate::api::param::ServerAddrs;
use crate::api::param::Strict;
use crate::api::param::ToServerAddrs;
use crate::api::storage::FDb;
use crate::api::Result;
use std::path::Path;
use url::Url;

impl ToServerAddrs<FDb> for &str {
	type Client = Db;

	fn to_server_addrs(self) -> Result<ServerAddrs> {
		Ok(ServerAddrs {
			endpoint: Url::parse(&format!("fdb://{self}"))?,
			strict: false,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}

impl ToServerAddrs<FDb> for &Path {
	type Client = Db;

	fn to_server_addrs(self) -> Result<ServerAddrs> {
		Ok(ServerAddrs {
			endpoint: Url::parse(&format!("fdb://{}", self.display()))?,
			strict: false,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}

impl<T> ToServerAddrs<FDb> for (T, Strict)
where
	T: AsRef<Path>,
{
	type Client = Db;

	fn to_server_addrs(self) -> Result<ServerAddrs> {
		Ok(ServerAddrs {
			endpoint: Url::parse(&format!("fdb://{}", self.0.as_ref().display()))?,
			strict: true,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}
