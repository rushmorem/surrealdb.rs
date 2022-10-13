use crate::Connection;

pub struct Health<'a, C: ?Sized> {
    conn: &'a mut C,
}
