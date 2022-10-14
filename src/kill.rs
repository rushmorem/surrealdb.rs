use crate::Connection;

pub struct Kill<'a, C: ?Sized> {
    conn: &'a mut C,
}
