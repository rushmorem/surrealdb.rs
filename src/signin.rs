use crate::{Connection, Result};
use serde::de::DeserializeOwned;
use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use std::pin::Pin;

pub struct Signin<'a, C: ?Sized, R> {
    conn: &'a mut C,
    response: PhantomData<R>,
}

impl<'a, C, R> IntoFuture for Signin<'a, C, R>
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
