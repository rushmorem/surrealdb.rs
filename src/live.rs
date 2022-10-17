use crate::{Connection, Result};
use std::future::{Future, IntoFuture};
use std::pin::Pin;
use uuid::Uuid;

pub struct Live<'a, C: ?Sized> {
    conn: &'a mut C,
}

impl<'a, C> IntoFuture for Live<'a, C>
where
    C: Connection + ?Sized,
{
    type Output = Result<Uuid>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Uuid>>>>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}
