use crate::{Connection, Record, Result, Table};
use serde::de::DeserializeOwned;
use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use std::pin::Pin;

pub struct Select<'a, C: ?Sized, T, R> {
    conn: &'a mut C,
    response_type: PhantomData<T>,
    response: PhantomData<R>,
}

impl<'a, C, R> IntoFuture for Select<'a, C, Record, R>
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

impl<'a, C, R> IntoFuture for Select<'a, C, Table, R>
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
