use crate::api::method::Method;
use crate::api::param::Param;
use crate::api::Connection;
use crate::api::Result;
use crate::api::Router;
use std::future::Future;
use std::future::IntoFuture;
use std::path::PathBuf;
use std::pin::Pin;

/// A database export future
#[derive(Debug)]
pub struct Export<'r, C: Connection> {
	pub(super) router: Result<&'r Router<C>>,
	pub(super) file: PathBuf,
}

impl<'r, Client> IntoFuture for Export<'r, Client>
where
	Client: Connection,
{
	type Output = Result<()>;
	type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + Sync + 'r>>;

	fn into_future(self) -> Self::IntoFuture {
		Box::pin(async {
			let mut conn = Client::new(Method::Export);
			conn.execute(self.router?, Param::file(self.file)).await
		})
	}
}
