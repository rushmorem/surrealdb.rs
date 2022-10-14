use crate::{Connection, Result};
use std::future::{Future, IntoFuture};
use std::pin::Pin;
use surrealdb::sql::Value;

pub struct Kill<'a, C: ?Sized> {
    conn: &'a mut C,
}

impl<'a, C> IntoFuture for Kill<'a, C>
where
    C: Connection + ?Sized,
{
    type Output = Result<Value>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Value>>>>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}
