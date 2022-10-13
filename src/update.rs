use crate::Connection;

pub struct Update<'a, C: ?Sized> {
    conn: &'a mut C,
}
