use dagwasm_dagio::Dagio;
use dagwasm_derivation::Derivation;
use dagwasm_memstore::MemStore;

#[tokio::test]
async fn get_derivation_outputs_derivation() -> anyhow::Result<()> {
    let r = get_derivation_outputs_derivation_impl().await;
    if let Some(e) = r.as_ref().err() {
        eprintln!("{e:#}");
    }
    r
}

async fn get_derivation_outputs_derivation_impl() -> anyhow::Result<()> {
    let mut ms = MemStore::default();
    let mut dagio = Dagio::from(&mut ms);

    // Set up derivation:
    let exec = dagio
        .write_file(dagwasm_guests::get_wasm_bytes("get_derivation")?)
        .await?;
    let input = dagio.write_file(b"").await?;

    let derivation = dagio.commit(Derivation { exec, input }).await?;

    // Execute derive:
    let output = dagwasm_host::derive(&mut ms, &derivation).await?;

    // Verify output is the derivation:
    assert_eq!(output, derivation);

    Ok(())
}
