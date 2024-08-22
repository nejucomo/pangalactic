/// The environment variable which influences the logging configuration
pub const ENV_NAME: &str = "PG_LOG";

const DEFAULT_DIRECTIVES: &str = "debug,cranelift_codegen=info,wasmtime_cranelift=info";

pub fn init() -> anyhow::Result<()> {
    use tracing_subscriber::EnvFilter;

    if matches!(std::env::var(ENV_NAME), Err(std::env::VarError::NotPresent)) {
        unsafe {
            std::env::set_var(ENV_NAME, DEFAULT_DIRECTIVES);
        }
    }

    let filter = EnvFilter::builder().with_env_var(ENV_NAME).from_env()?;

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .try_init()
        .map_err(|e| anyhow::anyhow!(e))
}

pub fn test_init() {
    use once_cell::sync::OnceCell;

    static INSTANCE: OnceCell<()> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        init().unwrap();
    });
}
