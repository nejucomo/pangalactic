pub fn init() -> anyhow::Result<()> {
    use tracing::Level;

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_writer(std::io::stderr)
        .try_init()
        .map_err(|e| anyhow::anyhow!(e))

    // for modname in QUIET_LIST {
    //     logger = logger.with_module_level(modname, log::LevelFilter::Warn);
    // }
}

pub fn test_init() {
    use once_cell::sync::OnceCell;

    static INSTANCE: OnceCell<()> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        init().unwrap();
    });
}

// const QUIET_LIST: &[&str] = &["cranelift_codegen", "wasmtime_cranelift"];
