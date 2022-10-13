use crate::Connection;

pub struct Change<'a, C: ?Sized> {
    conn: &'a mut C,
}
