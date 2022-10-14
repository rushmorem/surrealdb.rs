use crate::{Cancel, Commit, Connection, Result, Surreal};
use std::future::{Future, IntoFuture};
use std::ops::{Deref, DerefMut};
use std::pin::Pin;

pub struct Begin<C: Connection> {
    client: Surreal<C>,
}

impl<C> IntoFuture for Begin<C>
where
    C: Connection,
{
    type Output = Result<Transaction<C>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Transaction<C>>>>>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}

pub struct Transaction<C: Connection> {
    client: Surreal<C>,
}

impl<C> Transaction<C>
where
    C: Connection,
{
    pub fn commit(self) -> Commit<C> {
        Commit {
            client: self.client,
        }
    }

    pub fn cancel(self) -> Cancel<C> {
        Cancel {
            client: self.client,
        }
    }
}

impl<C: Connection> Deref for Transaction<C> {
    type Target = Surreal<C>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl<C: Connection> DerefMut for Transaction<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}
