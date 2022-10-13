use crate::Connection;
use std::marker::PhantomData;

pub struct Signup<'a, C: ?Sized, R> {
    conn: &'a mut C,
    response: PhantomData<R>,
}
