use crate::Connection;

pub struct Delete<'a, C: ?Sized> {
    conn: &'a mut C,
}
