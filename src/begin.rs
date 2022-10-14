use crate::Connection;

pub struct Begin<'a, C: ?Sized> {
    conn: &'a mut C,
}
