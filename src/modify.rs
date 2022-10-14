use crate::{Connection, Patches, Record, Result, Table};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use std::pin::Pin;

pub struct Modify<'a, C: ?Sized, T, R> {
    conn: &'a mut C,
    response_type: PhantomData<T>,
    response: PhantomData<R>,
}

impl<'a, C, R> IntoFuture for Modify<'a, C, Record, R>
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

impl<'a, C, R> IntoFuture for Modify<'a, C, Table, R>
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

impl<'a, C, T, R> Modify<'a, C, T, R> {
    pub fn patches<D>(self, data: Vec<D>) -> Patches<'a, C, T, R>
    where
        D: Serialize,
    {
        todo!()
    }
}
