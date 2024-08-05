use crate::options::{Options, Runnable};

pub async fn run() -> anyhow::Result<()> {
    init_logging()?;
    let logargs = std::env::args().collect::<Vec<_>>();
    tracing::debug!(?logargs);
    let opts = Options::parse();
    tracing::debug!(?opts);
    if let Some(path) = opts.run().await? {
        println!("{path}");
    }
    Ok(())
}

fn init_logging() -> anyhow::Result<()> {
    use tracing::Level;

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_writer(std::io::stderr)
        .try_init()
        .map_err(|e| anyhow::anyhow!(e))
}
