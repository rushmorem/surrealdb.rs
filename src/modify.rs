use crate::Connection;

pub struct Modify<'a, C: ?Sized> {
    conn: &'a mut C,
}
