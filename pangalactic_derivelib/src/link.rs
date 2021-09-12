use crate::LinkPrim;

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct LinkHandle(LinkPrim);

#[derive(Debug, PartialEq, Eq, num_derive::FromPrimitive)]
pub enum LinkType {
    File = 0,
    Dir = 1,
}

impl LinkHandle {
    pub fn link_type(&self) -> LinkType {
        use num_traits::FromPrimitive;
        let typeprim = unsafe { crate::bindings::link_type(self.0) };
        LinkType::from_i64(typeprim).unwrap()
    }

    pub(crate) fn unwrap_prim(self) -> LinkPrim {
        self.0
    }
}
