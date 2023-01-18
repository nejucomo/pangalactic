use crate::{bindings, prim, Link};

pub fn write_bytes(bytes: &[u8]) -> Link {
    let w = ByteWriter::open();
    w.write(bytes);
    w.commit()
}

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

impl std::io::Write for ByteWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        ByteWriter::write(self, buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
