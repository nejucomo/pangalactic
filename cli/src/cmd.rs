use pangalactic_dagio::Dagio;
use pangalactic_store_dirdb::DirDbStore;

pub async fn store_insert() -> anyhow::Result<()> {
    let mut dagio = Dagio::from(DirDbStore::default());
    let mut r = tokio::io::stdin();
    let mut w = dagio.open_file_writer().await?;
    tokio::io::copy(&mut r, &mut w).await?;
    let link = dagio.commit_file_writer(w).await?;
    println!("{link}");
    Ok(())
}
