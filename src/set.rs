use crate::{Connection, Result};
use std::future::{Future, IntoFuture};
use std::pin::Pin;

pub struct Set<'a, C: ?Sized> {
    conn: &'a mut C,
}

impl<'a, C> IntoFuture for Set<'a, C>
where
    C: Connection + ?Sized,
{
    type Output = Result<()>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<()>>>>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}
