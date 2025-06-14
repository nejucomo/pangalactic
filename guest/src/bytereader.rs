use crate::{bindings, prim};

pub struct ByteReader {
    handle: prim::HandleByteReader,
    trace: bool,
}

impl ByteReader {
    pub(crate) fn wrap_handle(handle: prim::HandleByteReader, trace: bool) -> Self {
        ByteReader { handle, trace }
    }

    pub fn read(&self, buf: &mut [u8]) -> usize {
        let read_amount = usize::try_from(unsafe {
            let (ptr, len) = crate::ptr::unpack_for_read(buf);
            bindings::byte_reader_read(self.handle, ptr, len)
        })
        .expect("ByteLen->usize failure");

        if self.trace {
            trace!(
                "{self:?} read: {:?}",
                String::from_utf8_lossy(&buf[..read_amount])
            );
        }
        assert!(read_amount <= buf.len());
        read_amount
    }

    pub fn read_to_vec(self) -> Vec<u8> {
        let mut v = vec![0u8; 64];
        let mut i = 0;

        let mut c = self.read(&mut v[i..]);
        i += c;
        while i == v.len() {
            v.resize_with(v.len() * 2, Default::default);
            c = self.read(&mut v[i..]);
            i += c;
        }
        v.truncate(i);
        if self.trace {
            trace!(
                "{self:?} read_to_vec -> {:?}",
                String::from_utf8_lossy(&v[..])
            );
        }
        v
    }
}

impl std::ops::Drop for ByteReader {
    fn drop(&mut self) {
        unsafe { bindings::byte_reader_close(self.handle) };
    }
}

impl std::fmt::Debug for ByteReader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "ByteReader({}{})",
            self.handle,
            if self.trace { "" } else { " no-trace" }
        )
    }
}

impl std::io::Read for ByteReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(ByteReader::read(self, buf))
    }
}
