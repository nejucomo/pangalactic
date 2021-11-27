use crate::{bindings, prim};

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct BufWriterHandle(prim::BufWriterHandle);

impl BufWriterHandle {
    pub fn new() -> Self {
        let prim = unsafe { bindings::new_file() };
        BufWriterHandle(prim)
    }
}
