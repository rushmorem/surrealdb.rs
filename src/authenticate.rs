use crate::Connection;

pub struct Authenticate<'a, C: ?Sized> {
    conn: &'a mut C,
}
