use crate::Connection;

pub struct Set<'a, C: ?Sized> {
    conn: &'a mut C,
}
