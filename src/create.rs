use crate::{Connection, Content, Record, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use std::pin::Pin;

pub struct Create<'a, C: ?Sized, R> {
    conn: &'a mut C,
    response: PhantomData<R>,
}

impl<'a, C, R> IntoFuture for Create<'a, C, R>
where
    C: Connection + ?Sized,
    R: DeserializeOwned,
{
    type Output = Result<R>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<R>>>>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}

impl<'a, C, R> Create<'a, C, R> {
    pub fn content<D>(self, data: D) -> Content<'a, C, Record, R>
    where
        D: Serialize,
    {
        todo!()
    }
}
