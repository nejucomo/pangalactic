use crate::LinkPrim;
pub use pangalactic_node::Kind;

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct LinkHandle(LinkPrim);

impl LinkHandle {
    pub fn kind(&self) -> Kind {
        let kindprim = unsafe { crate::bindings::link_kind(self.0) };
        match kindprim {
            0 => Kind::File,
            1 => Kind::Dir,
            _ => panic!("Invalid kindprim: {}", kindprim),
        }
    }

    pub(crate) fn unwrap_prim(self) -> LinkPrim {
        self.0
    }
}
