use dagwasm_dagio::Dagio;
use dagwasm_dir::Directory;
use dagwasm_memstore::MemStore;

#[tokio::test]
async fn derivation_is_dir() -> anyhow::Result<()> {
    let r = derivation_is_dir_impl().await;
    if let Some(e) = r.as_ref().err() {
        eprintln!("{e:#}");
    }
    r
}

async fn derivation_is_dir_impl() -> anyhow::Result<()> {
    let mut ms = MemStore::default();
    let mut dagio = Dagio::from(&mut ms);

    // Set up derivation:
    let exec = dagio
        .write_file(dagwasm_guests::get_wasm_bytes("test_derivation_is_dir")?)
        .await?;
    let empty = dagio.write_file(b"").await?;
    let derivation = dagio
        .commit(Directory::from_iter([("exec", exec), ("input", empty)]))
        .await?;

    // Execute derive:
    let _ = dagwasm_host::derive(&mut ms, &derivation).await?;

    Ok(())
}
