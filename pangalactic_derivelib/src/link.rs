use crate::{bindings, prim, BufReaderHandle};
pub use pangalactic_node::Kind;

#[derive(Debug, derive_more::From)]
pub struct LinkHandle(prim::LinkHandle);

impl LinkHandle {
    pub fn kind(&self) -> Kind {
        let kindprim = unsafe { bindings::link_kind(self.0) };
        match kindprim {
            0 => Kind::File,
            1 => Kind::Dir,
            _ => panic!("Invalid kindprim: {}", kindprim),
        }
    }

    pub fn load_file(&self) -> BufReaderHandle {
        let primread = unsafe { bindings::link_load_file(self.0) };
        BufReaderHandle::from(primread)
    }

    pub(crate) fn unwrap_prim(self) -> prim::LinkHandle {
        self.0
    }
}

impl PartialEq for LinkHandle {
    fn eq(&self, other: &LinkHandle) -> bool {
        prim::bool_host2guest(unsafe { bindings::link_eq(self.0, other.0) })
    }
}

impl Eq for LinkHandle {}
