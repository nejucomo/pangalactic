macro_rules! trace {
    ( $x:expr ) => {{
        log::trace!("{}...", stringify!($x));
        let res = $x;
        log::trace!(
            "{} =>\n  {}",
            stringify!($x),
            format!("{:#?}", &res).replace("\n", "\n  ")
        );

        res
    }};
}
