use dagwasm_dagio::Dagio;
use dagwasm_memstore::MemStore;
use dagwasm_schemata::{Attestation, Plan};
use std::future::Future;

#[tokio::test]
async fn plan_is_dir() -> anyhow::Result<()> {
    verify_guests(
        &["test_plan_is_dir", "test_bindings_plan_is_dir"],
        b"",
        |_, _, _| async { Ok(()) },
    )
    .await
}

#[tokio::test]
async fn get_plan_outputs_plan() -> anyhow::Result<()> {
    verify_guests(
        &["get_plan"],
        b"",
        |_dagio, _plan, attestation| async move {
            assert_eq!(attestation.plan, attestation.output);
            Ok(())
        },
    )
    .await
}

#[tokio::test]
async fn identity() -> anyhow::Result<()> {
    verify_guests(&["identity"], b"", |_dagio, plan, attestation| async move {
        assert_eq!(plan.input, attestation.output);
        Ok(())
    })
    .await
}

#[tokio::test]
async fn input_is_hello_world() -> anyhow::Result<()> {
    verify_guests(
        &["test_input_is_hello_world"],
        b"Hello World!",
        |_, _, _| async { Ok(()) },
    )
    .await
}

async fn verify_guests<const K: usize, F, Fut>(
    guests: &[&str],
    input_bytes: &[u8; K],
    verify: F,
) -> anyhow::Result<()>
where
    F: Fn(Dagio<MemStore>, Plan<MemStore>, Attestation<MemStore>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let r = verify_guests_inner(guests, input_bytes, verify).await;
    if let Some(e) = r.as_ref().err() {
        eprintln!("{e:#}");
    }
    r
}

async fn verify_guests_inner<const K: usize, F, Fut>(
    guests: &[&str],
    input_bytes: &[u8; K],
    verify: F,
) -> anyhow::Result<()>
where
    F: Fn(Dagio<MemStore>, Plan<MemStore>, Attestation<MemStore>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    for guest in guests {
        verify_guest_inner(guest, input_bytes, &verify).await?;
    }
    Ok(())
}

async fn verify_guest_inner<const K: usize, F, Fut>(
    guest: &str,
    input_bytes: &[u8; K],
    verify: F,
) -> anyhow::Result<()>
where
    F: Fn(Dagio<MemStore>, Plan<MemStore>, Attestation<MemStore>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let mut dagio = Dagio::from(MemStore::default());

    let plan = {
        // Set up plan:
        let exec = dagio
            .write_file(dagwasm_guests::get_wasm_bytes(guest)?)
            .await?;
        let input = dagio.write_file(input_bytes.as_slice()).await?;

        dagio.commit(Plan { exec, input }).await?
    };

    // Execute derive:
    let (mut dagio, attestation) = dagwasm_host::derive(dagio, &plan).await?;

    let att: Attestation<MemStore> = dagio.read(&attestation).await?;
    let der: Plan<MemStore> = dagio.read(&att.plan).await?;

    // Verify
    verify(dagio, der, att).await?;

    Ok(())
}
