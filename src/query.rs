use crate::Connection;

pub struct Query<'a, C: ?Sized> {
    conn: &'a mut C,
}
