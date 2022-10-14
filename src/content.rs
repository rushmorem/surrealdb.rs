use crate::param::Resource;
use crate::{Connection, Record, Result, Table};
use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use std::pin::Pin;

pub struct Content<'a, C: ?Sized, T, R> {
    conn: &'a mut C,
    response_type: PhantomData<T>,
    response: PhantomData<R>,
}

impl<'a, C, R> IntoFuture for Content<'a, C, Record, R>
where
    C: Connection + ?Sized,
{
    type Output = Result<R>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<R>>>>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}

impl<'a, C, R> IntoFuture for Content<'a, C, Table, R>
where
    C: Connection + ?Sized,
{
    type Output = Result<Vec<R>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Vec<R>>>>>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}
