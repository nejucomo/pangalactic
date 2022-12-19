use dagwasm_dagio::{Dagio, LinkFor};
use dagwasm_derivation::Derivation;
use dagwasm_memstore::MemStore;
use std::future::Future;

async fn verify_guest<F, Fut>(guest: &str, verify: F) -> anyhow::Result<()>
where
    F: FnOnce(LinkFor<MemStore>, LinkFor<MemStore>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let r = verify_guest_inner(guest, verify).await;
    if let Some(e) = r.as_ref().err() {
        eprintln!("{e:#}");
    }
    r
}

async fn verify_guest_inner<F, Fut>(guest: &str, verify: F) -> anyhow::Result<()>
where
    F: FnOnce(LinkFor<MemStore>, LinkFor<MemStore>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let mut dagio = Dagio::from(MemStore::default());

    // Set up derivation:
    let exec = dagio
        .write_file(dagwasm_guests::get_wasm_bytes(guest)?)
        .await?;
    let input = dagio.write_file(b"").await?;

    let derivation = dagio.commit(Derivation { exec, input }).await?;

    // Execute derive:
    let output = dagwasm_host::derive(dagio, &derivation).await?;

    // Verify
    verify(derivation, output).await?;

    Ok(())
}

#[tokio::test]
async fn derivation_is_dir() -> anyhow::Result<()> {
    verify_guest("test_derivation_is_dir", |_, _| async { Ok(()) }).await
}

#[tokio::test]
async fn get_derivation_outputs_derivation() -> anyhow::Result<()> {
    verify_guest("get_derivation", |derivation, output| async move {
        assert_eq!(derivation, output);
        Ok(())
    })
    .await
}
