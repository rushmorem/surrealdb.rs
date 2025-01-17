use crate::api::method::Method;
use std::future::Future;
use std::pin::Pin;
use crate::api::param::Param;
use crate::api::Connection;
use crate::api::Result;
use crate::api::Router;
use std::future::IntoFuture;
use surrealdb::sql::Value;

#[derive(Debug)]
pub struct UseDb<'r, C: Connection> {
    pub(super) router: Result<&'r Router<C>>,
    pub(super) db: String,
}

impl<'r, Client> IntoFuture for UseDb<'r, Client>
where
    Client: Connection,
{
    type Output = Result<()>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + Sync + 'r>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let mut conn = Client::new(Method::Use);
            conn.execute(self.router?, Param::new(vec![Value::None, self.db.into()]))
                .await
        })
    }
}
