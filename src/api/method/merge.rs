use crate::api::method::Method;
use crate::api::param::from_json;
use crate::api::param::DbResource;
use crate::api::param::Param;
use crate::api::param::Range;
use crate::api::Connection;
use crate::api::Result;
use crate::api::Router;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use std::future::Future;
use std::future::IntoFuture;
use std::marker::PhantomData;
use std::pin::Pin;
use surrealdb::sql::Id;

/// A merge future
#[derive(Debug)]
pub struct Merge<'r, C: Connection, D, R> {
	pub(super) router: Result<&'r Router<C>>,
	pub(super) resource: Result<DbResource>,
	pub(super) range: Option<Range<Id>>,
	pub(super) content: D,
	pub(super) response_type: PhantomData<R>,
}

impl<'r, C, D, R> Merge<'r, C, D, R>
where
	C: Connection,
	D: Serialize,
{
	fn split(self) -> Result<(&'r Router<C>, Method, Param)> {
		let resource = self.resource?;
		let param = match self.range {
			Some(range) => resource.with_range(range)?,
			None => resource.into(),
		};
		let content = json!(self.content);
		let param = Param::new(vec![param, from_json(content)]);
		Ok((self.router?, Method::Merge, param))
	}
}

impl<'r, Client, D, R> IntoFuture for Merge<'r, Client, D, R>
where
	Client: Connection,
	D: Serialize,
	R: DeserializeOwned + Send + Sync,
{
	type Output = Result<R>;
	type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + Sync + 'r>>;

	fn into_future(self) -> Self::IntoFuture {
		let result = self.split();
		Box::pin(async move {
			let (router, method, param) = result?;
			let mut conn = Client::new(method);
			conn.execute(router, param).await
		})
	}
}
