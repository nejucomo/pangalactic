mod store;

use tokio::io;

use crate::{
    options::{Destination, Source},
    store::{CliDagio, CliLink},
};

pub use self::store::StoreCmds;

pub async fn store_put() -> anyhow::Result<()> {
    let mut dagio = CliDagio::default();
    let mut r = io::stdin();
    let mut w = dagio.open_file_writer().await?;
    io::copy(&mut r, &mut w).await?;
    let link = dagio.commit(w).await?;
    println!("{link}");
    Ok(())
}

pub async fn store_get(link: &CliLink) -> anyhow::Result<()> {
    use crate::store::CliReader;

    let dagio = CliDagio::default();
    let mut r: CliReader = dagio.load(link).await?;
    let mut w = io::stdout();
    io::copy(&mut r, &mut w).await?;
    Ok(())
}

pub async fn store_xfer(source: &Source, dest: &Destination) -> anyhow::Result<()> {
    StoreCmds::default().xfer(source, dest).await
}
