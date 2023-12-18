pub async fn store_insert() -> anyhow::Result<()> {
    use pangalactic_store::Store;
    use pangalactic_store_mem::MemStore;

    let mut store = MemStore::default();
    let mut r = tokio::io::stdin();
    let mut w = store.open_writer().await?;
    tokio::io::copy(&mut r, &mut w).await?;
    let cid = store.commit_writer(w).await?;
    println!("{cid:?}");
    Ok(())
}
