use crate::LinkPrim;

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct LinkHandle(LinkPrim);

#[derive(Debug, PartialEq, Eq, num_derive::FromPrimitive)]
pub enum LinkKind {
    File = 0,
    Dir = 1,
}

impl LinkHandle {
    pub fn kind(&self) -> LinkKind {
        use num_traits::FromPrimitive;
        let kindprim = unsafe { crate::bindings::link_kind(self.0) };
        LinkKind::from_i32(kindprim).unwrap()
    }

    pub(crate) fn unwrap_prim(self) -> LinkPrim {
        self.0
    }
}
