use crate::{bindings, prim};

#[derive(Debug)]
pub struct ByteReader(prim::HandleByteReader);

impl ByteReader {
    pub(crate) fn wrap_handle(handle: prim::HandleByteReader) -> Self {
        ByteReader(handle)
    }

    pub fn read(&self, buf: &mut [u8]) -> usize {
        let ptr = buf.as_mut_ptr() as i64; // FIXME: Safe conversion which panics on overflow.
        let len = prim::ByteLen::try_from(buf.len()).expect("usize->prim::ByteLen failure");

        let read_amount = usize::try_from(unsafe { bindings::byte_reader_read(self.0, ptr, len) })
            .expect("u64->usize failure");
        assert!(read_amount <= buf.len());
        read_amount
    }

    pub fn read_to_vec(self) -> Vec<u8> {
        let mut v = vec![0u8; 64];
        let mut i = 0;

        let mut c = self.read(&mut v[i..]);
        while c == v.len() - i {
            v.resize_with(v.len() * 2, Default::default);
            i += c;
            c = self.read(&mut v[i..]);
        }
        v.shrink_to_fit();
        v
    }
}

impl std::ops::Drop for ByteReader {
    fn drop(&mut self) {
        unsafe { bindings::byte_reader_close(self.0) };
    }
}
