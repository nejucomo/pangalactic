pub fn init(level: log::Level) {
    let mut logger = simple_logger::SimpleLogger::new().with_level(level.to_level_filter());

    for modname in QUIET_LIST {
        logger = logger.with_module_level(modname, log::LevelFilter::Warn);
    }

    logger.init().unwrap();
}

pub fn test_init() {
    use once_cell::sync::OnceCell;

    static INSTANCE: OnceCell<()> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        init(log::Level::Debug);
    });
}

const QUIET_LIST: &[&str] = &["cranelift_codegen", "wasmtime_cranelift"];
