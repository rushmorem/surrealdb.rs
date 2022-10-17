use crate::{Connection, Result};
use std::future::{Future, IntoFuture};
use std::pin::Pin;
use surrealdb::sql::statements::InfoStatement;

pub struct Info<'a, C: ?Sized> {
    conn: &'a mut C,
}

impl<'a, C> IntoFuture for Info<'a, C>
where
    C: Connection + ?Sized,
{
    type Output = Result<InfoStatement>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<InfoStatement>>>>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}
