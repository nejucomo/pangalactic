use pangalactic_dagio::Dagio;
use pangalactic_store_mem::MemStore;

pub async fn store_insert() -> anyhow::Result<()> {
    use pangalactic_serialization::AsyncSerialize;

    let mut dagio = Dagio::from(MemStore::default());
    let mut r = tokio::io::stdin();
    let mut w = dagio.open_file_writer().await?;
    tokio::io::copy(&mut r, &mut w).await?;
    let cid = dagio.commit_file_writer(w).await?;
    let mut ser = vec![];
    cid.write_into(&mut ser).await?;
    let enc = pangalactic_b64::encode(ser);
    println!("{enc}");
    Ok(())
}
