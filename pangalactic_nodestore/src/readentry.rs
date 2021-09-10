use crate::DirFor;
use pangalactic_store::{ReaderOf, Store};

pub enum ReadEntry<S>
where
    S: Store,
{
    Dir(DirFor<S>),
    FileStream(ReaderOf<S>),
}
