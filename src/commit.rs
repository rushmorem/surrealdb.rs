use crate::{Connection, Result, Surreal};
use std::future::{Future, IntoFuture};
use std::pin::Pin;

pub struct Commit<C: Connection> {
    pub(crate) client: Surreal<C>,
}

impl<C> IntoFuture for Commit<C>
where
    C: Connection,
{
    type Output = Result<Surreal<C>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Surreal<C>>>>>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}
