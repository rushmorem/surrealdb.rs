use crate::Connection;

pub struct Live<'a, C: ?Sized> {
    conn: &'a mut C,
}
