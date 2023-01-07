use crate::{bindings, prim, Link};

#[derive(Debug)]
pub struct ByteWriter(prim::HandleByteWriter);

impl ByteWriter {
    pub fn open() -> Self {
        ByteWriter(unsafe { bindings::byte_writer_open() })
    }

    pub fn write(&self, bytes: &[u8]) {
        unsafe {
            let (ptr, len) = crate::ptr::unpack_for_write(bytes);
            bindings::byte_writer_write(self.0, ptr, len);
        }
    }

    pub fn commit(self) -> Link {
        unsafe { Link::wrap_handle(bindings::byte_writer_commit(self.0)) }
    }
}
