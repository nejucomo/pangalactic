use crate::{prim, ReadHandle};
pub use pangalactic_node::Kind;

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct LinkHandle(prim::Link);

impl LinkHandle {
    pub fn kind(&self) -> Kind {
        let kindprim = unsafe { crate::bindings::link_kind(self.0) };
        match kindprim {
            0 => Kind::File,
            1 => Kind::Dir,
            _ => panic!("Invalid kindprim: {}", kindprim),
        }
    }

    pub fn load_file(&self) -> ReadHandle {
        let primread = unsafe { crate::bindings::load_file(self.0) };
        ReadHandle::from(primread)
    }

    pub(crate) fn unwrap_prim(self) -> prim::Link {
        self.0
    }
}
