use crate::{bindings, prim, LinkHandle};

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct BufWriterHandle(prim::BufWriterHandle);

impl BufWriterHandle {
    pub fn new() -> Self {
        let prim = unsafe { bindings::bufwriter_new() };
        BufWriterHandle(prim)
    }

    pub fn write(&self, buf: &[u8]) {
        let (bufptr, buflen) = prim::bytes_guest2host(buf);
        unsafe {
            bindings::bufwriter_write(self.0, bufptr, buflen);
        }
    }

    pub fn commit(self) -> LinkHandle {
        LinkHandle::from(unsafe { bindings::bufwriter_commit(self.0) })
    }
}
