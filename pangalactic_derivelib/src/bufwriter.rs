use crate::{bindings, prim};

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct BufWriterHandle(prim::BufWriterHandle);

impl BufWriterHandle {
    pub fn new() -> Self {
        let prim = unsafe { bindings::new_file() };
        BufWriterHandle(prim)
    }

    pub fn write(&self, buf: &[u8]) {
        use std::convert::TryInto;

        let bufptr = buf.as_ptr() as i64; // BUG: How to do this without overflow which would cause memory corruption?
        let buflenu: usize = buf.len();
        let buflen: prim::MemLen = buflenu.try_into().unwrap();
        unsafe {
            bindings::bufwriter_write(self.0, bufptr, buflen);
        }
    }
}
