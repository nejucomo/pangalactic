use crate::{bindings, prim};
use dagwasm_linkkind::LinkKind;

#[derive(Debug)]
pub struct Link(prim::HandleLink);

impl Link {
    #[doc(hidden)]
    pub unsafe fn wrap_handle(h: prim::HandleLink) -> Self {
        Link(h)
    }

    pub fn kind(&self) -> LinkKind {
        let lkprim: prim::LinkKind = unsafe { bindings::link_get_kind(self.0) };
        LinkKind::try_from(lkprim).unwrap()
    }
}
