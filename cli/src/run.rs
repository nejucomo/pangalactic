use crate::options::Options;

pub async fn run() -> anyhow::Result<()> {
    let opts = Options::parse();
    init_logging()?;
    tracing::debug!(options = ?opts);
    opts.run().await
}

fn init_logging() -> anyhow::Result<()> {
    use tracing::Level;

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_writer(std::io::stderr)
        .try_init()
        .map_err(|e| anyhow::anyhow!(e))
}
