use crate::options::{Options, Runnable};

pub async fn run() -> anyhow::Result<()> {
    pangalactic_log::init()?;

    let logargs = std::env::args().collect::<Vec<_>>();
    tracing::debug!(?logargs);
    let opts = Options::parse();
    tracing::debug!(?opts);
    if let Some(disp) = opts.run().await? {
        println!("{disp}");
    }
    Ok(())
}
