use crate::Connection;

pub struct Version<'a, C: ?Sized> {
    conn: &'a mut C,
}
