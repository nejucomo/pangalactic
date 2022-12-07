use dagwasm_dagio::Dagio;
use dagwasm_dir::Directory;
use dagwasm_memstore::MemStore;

#[tokio::test]
async fn get_derivation_outputs_derivation() -> anyhow::Result<()> {
    let mut ms = MemStore::default();
    let mut dagio = Dagio::from(&mut ms);

    // Set up derivation:
    let exec = dagio
        .write_file(dagwasm_guests::get_wasm_bytes("get_derivation")?)
        .await?;
    dbg!(&exec);
    let empty = dagio.write_file(b"").await?;
    dbg!(&empty);
    let derivation = dagio
        .commit_directory(&Directory::from_iter([("exec", exec), ("input", empty)]))
        .await?;
    dbg!(&derivation);

    // Execute derive:
    let output = dagwasm_host::derive(&mut ms, &derivation).await?;
    dbg!(&output);

    // Verify output is the derivation:
    assert_eq!(output, derivation);

    Ok(())
}
