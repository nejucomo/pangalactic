use crate::{bindings, prim};

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct BufReaderHandle(prim::BufReaderHandle);

impl BufReaderHandle {
    pub fn read(&self, buf: &mut [u8]) -> usize {
        use std::convert::TryInto;

        let (bufptr, buflen) = prim::bytes_guest2host_mut(buf);
        let w = unsafe { bindings::bufreader_read(self.0, bufptr, buflen) };
        w.try_into().unwrap()
    }
}
