use crate::Connection;

pub struct Create<'a, C: ?Sized> {
    conn: &'a mut C,
}
