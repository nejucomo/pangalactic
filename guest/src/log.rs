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

#[macro_export]
macro_rules! unwrap {
    ( Option $x:expr) => {
        match $x {
            Some(v) => v,
            None => {
                $crate::fail!("failed to unwrap {:?}", stringify!($x));
            }
        }
    };

    ( Result $x:expr) => {
        match $x {
            Ok(v) => v,
            Err(error) => {
                $crate::fail!("failed to unwrap {:?}: {error:?}", stringify!($x));
            }
        }
    };
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
    unsafe {
        let (ptr, len) = crate::ptr::unpack_for_write(bytes);
        crate::bindings::log(ptr, len);
    }
}
