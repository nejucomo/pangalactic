use crate::{prim, BufReaderHandle};
pub use pangalactic_node::Kind;

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct LinkHandle(prim::LinkHandle);

impl LinkHandle {
    pub fn kind(&self) -> Kind {
        let kindprim = unsafe { crate::bindings::link_kind(self.0) };
        match kindprim {
            0 => Kind::File,
            1 => Kind::Dir,
            _ => panic!("Invalid kindprim: {}", kindprim),
        }
    }

    pub fn load_file(&self) -> BufReaderHandle {
        let primread = unsafe { crate::bindings::load_file(self.0) };
        BufReaderHandle::from(primread)
    }

    pub(crate) fn unwrap_prim(self) -> prim::LinkHandle {
        self.0
    }
}
