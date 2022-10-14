use crate::{Connection, Content, Record, Result, Table};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use std::pin::Pin;

pub struct Update<'a, C: ?Sized, T, R> {
    conn: &'a mut C,
    response_type: PhantomData<T>,
    response: PhantomData<R>,
}

impl<'a, C, R> IntoFuture for Update<'a, C, Record, R>
where
    C: Connection + ?Sized,
    R: DeserializeOwned,
{
    type Output = Result<Option<R>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Option<R>>>>>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}

impl<'a, C, R> IntoFuture for Update<'a, C, Table, R>
where
    C: Connection + ?Sized,
    R: DeserializeOwned,
{
    type Output = Result<Vec<R>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Vec<R>>>>>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}

impl<'a, C, T, R> Update<'a, C, T, R> {
    pub fn content<D>(self, data: D) -> Content<'a, C, T, R>
    where
        D: Serialize,
    {
        todo!()
    }
}
