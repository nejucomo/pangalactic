pub fn init(level: log::Level) {
    simple_logger::init_with_level(level).unwrap();
}

pub fn test_init() {
    use once_cell::sync::OnceCell;

    static INSTANCE: OnceCell<()> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        init(log::Level::Trace);
    });
}
