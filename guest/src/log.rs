use crate::{bindings, prim};

#[macro_export]
macro_rules! log {
    ( $msg:expr ) => {
        $crate::log_str($msg);
    };

    ( $tmpl:expr, $( $arg:expr ),* ) => {
        $crate::log_str(&format!( $tmpl, $( $arg ),* ));
    }
}

macro_rules! trace {
    ( $msg:expr ) => {
        $crate::log_str_inner($msg);
    };

    ( $tmpl:expr, $( $arg:expr ),* ) => {
        $crate::log::log_str_inner(&format!( $tmpl, $( $arg ),* ));
    }
}

pub fn log_str(msg: &str) {
    log_str_raw("application", msg);
}

pub(crate) fn log_str_inner(msg: &str) {
    log_str_raw("API trace  ", msg);
}

fn log_str_raw(prefix: &str, msg: &str) {
    let buf = format!("[{prefix}] {msg}");
    let bytes = buf.as_bytes();
    let ptr = bytes.as_ptr() as prim::PtrWrite; // FIXME: safe cast handling overflow.
    let len = prim::ByteLen::try_from(bytes.len()).unwrap();
    unsafe { bindings::log(ptr, len) };
}
