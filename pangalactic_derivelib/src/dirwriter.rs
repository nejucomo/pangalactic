use crate::{bindings, prim, LinkHandle};

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct DirWriterHandle(prim::DirWriterHandle);

impl DirWriterHandle {
    pub fn new() -> Self {
        let prim = unsafe { bindings::dirwriter_new() };
        DirWriterHandle(prim)
    }

    pub fn add_link(&self, name: &str, link: LinkHandle) {
        todo!("{:?}.add_link{:?}", self, (name, link));
    }

    pub fn commit(self) -> LinkHandle {
        todo!("{:?}.commit()", self);
    }
}
