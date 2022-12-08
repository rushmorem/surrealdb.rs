use crate::api::method::Method;
use crate::api::param::Param;
use crate::api::Connection;
use crate::api::Result;
use crate::api::Router;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::future::IntoFuture;
use std::marker::PhantomData;
use std::pin::Pin;
use surrealdb::sql::Value;

/// A signup future
#[derive(Debug)]
pub struct Signup<'r, C: Connection, R> {
	pub(super) router: Result<&'r Router<C>>,
	pub(super) credentials: Result<Value>,
	pub(super) response_type: PhantomData<R>,
}

impl<'r, Client, R> IntoFuture for Signup<'r, Client, R>
where
	Client: Connection,
	R: DeserializeOwned + Send + Sync,
{
	type Output = Result<R>;
	type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + Sync + 'r>>;

	fn into_future(self) -> Self::IntoFuture {
		Box::pin(async move {
			let mut conn = Client::new(Method::Signup);
			conn.execute(self.router?, Param::new(vec![self.credentials?])).await
		})
	}
}
