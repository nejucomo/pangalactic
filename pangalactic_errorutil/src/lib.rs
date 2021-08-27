#[macro_export]
macro_rules! io_error {
    ( $kind:expr, $tmpl:expr, $( $arg:expr ),* ) => {
        std::io::Error::new($kind, format!($tmpl, $( $arg ),* ))
    }
}

#[macro_export]
macro_rules! ok_or_io_error {
    ( $opt:expr, $kind:expr, $tmpl:expr, $( $arg:expr ),* ) => {
        match $opt {
            Some(x) => Ok(x),
            None => Err($crate::io_error!($kind, $tmpl, $( $arg ),* )),
        }
    }
}
