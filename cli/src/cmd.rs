use crate::store::CliDagio;

pub async fn store_put() -> anyhow::Result<()> {
    let mut dagio = CliDagio::default();
    let mut r = tokio::io::stdin();
    let mut w = dagio.open_file_writer().await?;
    tokio::io::copy(&mut r, &mut w).await?;
    let link = dagio.commit_file_writer(w).await?;
    println!("{link}");
    Ok(())
}
