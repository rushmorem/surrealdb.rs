use crate::Connection;

pub struct Export<'a, C: ?Sized> {
    conn: &'a mut C,
}
