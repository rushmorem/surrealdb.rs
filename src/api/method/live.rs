use crate::api::method::Method;
use crate::api::param::Param;
use crate::api::Connection;
use crate::api::Result;
use crate::api::Router;
use std::future::Future;
use std::future::IntoFuture;
use std::pin::Pin;
use surrealdb::sql::Table;
use surrealdb::sql::Uuid;
use surrealdb::sql::Value;

/// A live query future
#[derive(Debug)]
pub struct Live<'r, C: Connection> {
	pub(super) router: Result<&'r Router<C>>,
	pub(super) table_name: String,
}

impl<'r, Client> IntoFuture for Live<'r, Client>
where
	Client: Connection,
{
	type Output = Result<Uuid>;
	type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + Sync + 'r>>;

	fn into_future(self) -> Self::IntoFuture {
		Box::pin(async move {
			let mut conn = Client::new(Method::Live);
			conn.execute(self.router?, Param::new(vec![Value::Table(Table(self.table_name))])).await
		})
	}
}
