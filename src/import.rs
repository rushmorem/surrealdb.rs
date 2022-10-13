use crate::Connection;

pub struct Import<'a, C: ?Sized> {
    conn: &'a mut C,
}
