use crate::Connection;

pub struct Invalidate<'a, C: ?Sized> {
    conn: &'a mut C,
}
