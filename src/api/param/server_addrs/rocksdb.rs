use crate::api::embedded::Db;
use crate::api::param::ServerAddrs;
use crate::api::param::Strict;
use crate::api::param::ToServerAddrs;
use crate::api::storage::File;
use crate::api::storage::RocksDb;
use crate::api::Result;
use std::path::Path;
use url::Url;

impl ToServerAddrs<RocksDb> for &str {
	type Client = Db;

	fn to_server_addrs(self) -> Result<ServerAddrs> {
		Ok(ServerAddrs {
			endpoint: Url::parse(&format!("rocksdb://{self}"))?,
			strict: false,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}

impl ToServerAddrs<RocksDb> for &Path {
	type Client = Db;

	fn to_server_addrs(self) -> Result<ServerAddrs> {
		Ok(ServerAddrs {
			endpoint: Url::parse(&format!("rocksdb://{}", self.display()))?,
			strict: false,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}

impl<T> ToServerAddrs<RocksDb> for (T, Strict)
where
	T: AsRef<Path>,
{
	type Client = Db;

	fn to_server_addrs(self) -> Result<ServerAddrs> {
		Ok(ServerAddrs {
			endpoint: Url::parse(&format!("rocksdb://{}", self.0.as_ref().display()))?,
			strict: true,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}

impl ToServerAddrs<File> for &str {
	type Client = Db;

	fn to_server_addrs(self) -> Result<ServerAddrs> {
		Ok(ServerAddrs {
			endpoint: Url::parse(&format!("file://{self}"))?,
			strict: false,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}

impl ToServerAddrs<File> for &Path {
	type Client = Db;

	fn to_server_addrs(self) -> Result<ServerAddrs> {
		Ok(ServerAddrs {
			endpoint: Url::parse(&format!("file://{}", self.display()))?,
			strict: false,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}

impl<T> ToServerAddrs<File> for (T, Strict)
where
	T: AsRef<Path>,
{
	type Client = Db;

	fn to_server_addrs(self) -> Result<ServerAddrs> {
		Ok(ServerAddrs {
			endpoint: Url::parse(&format!("file://{}", self.0.as_ref().display()))?,
			strict: true,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}
