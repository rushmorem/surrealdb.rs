use crate::{Connection, Result};
use std::future::{Future, IntoFuture};
use std::pin::Pin;

pub struct Version<'a, C: ?Sized> {
    conn: &'a mut C,
}

impl<'a, C> IntoFuture for Version<'a, C>
where
    C: Connection + ?Sized,
{
    type Output = Result<String>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<String>>>>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}
