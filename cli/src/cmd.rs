use crate::store::{CliDagio, CliLink, CliPath};

pub async fn store_put() -> anyhow::Result<()> {
    let mut dagio = CliDagio::default();
    let mut r = tokio::io::stdin();
    let mut w = dagio.open_file_writer().await?;
    tokio::io::copy(&mut r, &mut w).await?;
    let link = dagio.commit_file_writer(w).await?;
    println!("{link}");
    Ok(())
}

pub async fn store_get(link: &CliLink) -> anyhow::Result<()> {
    let mut dagio = CliDagio::default();
    let mut r = dagio.open_file_reader(link).await?;
    let mut w = tokio::io::stdout();
    tokio::io::copy(&mut r, &mut w).await?;
    Ok(())
}

pub async fn store_copy(_source: CliPath, _dest: CliPath) -> anyhow::Result<()> {
    todo!();
}
