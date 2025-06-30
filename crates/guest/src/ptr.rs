use crate::prim::{ByteLen, PtrRead, PtrWrite};

pub(crate) unsafe fn unpack_for_write(bytes: &[u8]) -> (PtrWrite, ByteLen) {
    let ptr = bytes.as_ptr() as PtrWrite; // FIXME: safe cast handling overflow.
    let len = ByteLen::try_from(bytes.len()).unwrap();
    (ptr, len)
}

pub(crate) unsafe fn unpack_for_read(bytes: &mut [u8]) -> (PtrRead, ByteLen) {
    let ptr = bytes.as_mut_ptr() as PtrRead; // FIXME: safe cast handling overflow.
    let len = ByteLen::try_from(bytes.len()).unwrap();
    (ptr, len)
}
