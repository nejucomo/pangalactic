use crate::{bindings, prim};

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct DirWriterHandle(prim::DirWriterHandle);

impl DirWriterHandle {
    pub fn new() -> Self {
        let prim = unsafe { bindings::dirwriter_new() };
        DirWriterHandle(prim)
    }
}
