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
        crate::log(&format!(
            "Read into &{ptr}[..{len}]: {:?}",
            String::from_utf8_lossy(&buf[..read_amount])
        ));
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
        crate::log(&format!(
            "read_to_vec -> {:?}",
            String::from_utf8_lossy(&v[..])
        ));
        v
    }
}

impl std::ops::Drop for ByteReader {
    fn drop(&mut self) {
        unsafe { bindings::byte_reader_close(self.0) };
    }
}
