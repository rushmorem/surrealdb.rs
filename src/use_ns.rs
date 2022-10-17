use crate::{Connection, Result, UseDb};
use std::future::{Future, IntoFuture};
use std::pin::Pin;

pub struct UseNs<'a, C: ?Sized> {
    conn: &'a mut C,
}

impl<'a, C> IntoFuture for UseNs<'a, C>
where
    C: Connection + ?Sized,
{
    type Output = Result<()>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<()>>>>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}

impl<'a, C> UseNs<'a, C>
where
    C: Connection + ?Sized,
{
    pub fn use_db(&'a mut self, db: &str) -> UseDb<'a, C> {
        todo!()
    }
}
