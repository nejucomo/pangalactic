use crate::{bindings, prim};

pub fn log(msg: &str) {
    let bytes = msg.as_bytes();
    let ptr = bytes.as_ptr() as prim::PtrWrite; // FIXME: safe cast handling overflow.
    let len = prim::ByteLen::try_from(msg.len()).unwrap();
    unsafe { bindings::log(ptr, len) };
}
