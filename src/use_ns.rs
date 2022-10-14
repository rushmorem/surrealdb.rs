use crate::{Connection, UseDb};

pub struct UseNs<'a, C: ?Sized> {
    conn: &'a mut C,
}

impl<'a, C> UseNs<'a, C>
where
    C: Connection + ?Sized,
{
    pub fn use_db(&'a mut self, db: &str) -> UseDb<'a, C> {
        todo!()
    }
}
