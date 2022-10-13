use crate::Connection;

pub struct Info<'a, C: ?Sized> {
    conn: &'a mut C,
}
