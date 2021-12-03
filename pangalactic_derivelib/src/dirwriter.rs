use crate::{bindings, prim, LinkHandle};

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct DirWriterHandle(prim::DirWriterHandle);

impl DirWriterHandle {
    pub fn new() -> Self {
        let prim = unsafe { bindings::dirwriter_new() };
        DirWriterHandle(prim)
    }

    pub fn add_link(&self, name: &str, referent: LinkHandle) {
        let linkprim = referent.unwrap_prim();
        let (nameptr, namelen) = prim::bytes_guest2host(name);
        unsafe {
            bindings::dirwriter_add_link(self.0, nameptr, namelen, linkprim);
        }
    }

    pub fn commit(self) -> LinkHandle {
        LinkHandle::from(unsafe { bindings::dirwriter_commit(self.0) })
    }
}
