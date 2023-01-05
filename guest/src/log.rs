use crate::{bindings, prim};

#[macro_export]
macro_rules! log {
    ( $msg:expr ) => {
        $crate::log_str(&std::fmt::format(format_args!($msg)));
    };

    ( $tmpl:expr, $( $arg:expr ),* ) => {
        $crate::log_str(&std::fmt::format(format_args!( $tmpl, $( $arg ),* )));
    }
}

#[macro_export]
macro_rules! fail {
    ( $msg:expr ) => {{
        $crate::log!($msg);
        panic!($msg)
    }};

    ( $tmpl:expr, $( $arg:expr ),* ) => {{
        $crate::log!( $tmpl, $( $arg ),* );
        panic!( $tmpl, $( $arg ),* )
    }}
}

macro_rules! trace {
    ( $msg:expr ) => {
        $crate::log_str_inner(&std::fmt::format(format_args!($msg)));
    };

    ( $tmpl:expr, $( $arg:expr ),* ) => {
        $crate::log::log_str_inner(&std::fmt::format(format_args!( $tmpl, $( $arg ),* )));
    }
}

pub fn log_str(msg: &str) {
    log_str_raw("exec", msg);
}

pub(crate) fn log_str_inner(msg: &str) {
    log_str_raw("api ", msg);
}

fn log_str_raw(prefix: &str, msg: &str) {
    let buf = format!("[{prefix}] {msg}");
    let bytes = buf.as_bytes();
    let ptr = bytes.as_ptr() as prim::PtrWrite; // FIXME: safe cast handling overflow.
    let len = prim::ByteLen::try_from(bytes.len()).unwrap();
    unsafe { bindings::log(ptr, len) };
}
