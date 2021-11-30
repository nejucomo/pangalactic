use crate::{bindings, prim};

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct BufReaderHandle(prim::BufReaderHandle);

impl BufReaderHandle {
    pub fn read(&self, buf: &mut [u8]) -> usize {
        use std::convert::TryInto;

        let bufptr = buf.as_mut_ptr() as i64; // BUG: How to do this without overflow which would cause memory corruption?
        let buflenu: usize = buf.len();
        let buflen: prim::MemLen = buflenu.try_into().unwrap();
        let w = unsafe { bindings::bufreader_read(self.0, bufptr, buflen) };
        w.try_into().unwrap()
    }
}
