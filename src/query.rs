use crate::{Connection, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use std::pin::Pin;

pub struct Query<'a, C: ?Sized, R> {
    conn: &'a mut C,
    response: PhantomData<R>,
}

impl<'a, C, R> Query<'a, C, R>
where
    C: Connection + ?Sized,
    R: DeserializeOwned,
{
    pub fn bind<T>(self, bindings: T) -> Bindings<'a, C, T, R>
    where
        T: Serialize,
    {
        todo!()
    }
}

impl<'a, C, R> IntoFuture for Query<'a, C, R>
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

pub struct Bindings<'a, C: ?Sized, T, R> {
    conn: &'a mut C,
    bindings: T,
    response: PhantomData<R>,
}

impl<'a, C, T, R> IntoFuture for Bindings<'a, C, T, R>
where
    C: Connection + ?Sized,
    T: Serialize,
    R: DeserializeOwned,
{
    type Output = Result<Vec<R>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Vec<R>>>>>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}
