use pangalactic_host::HostLayerExt;
use pangalactic_schemata::{Attestation, Plan};
use pangalactic_store::Store;
use std::future::Future;

mod memtree;
mod teststore;

use self::memtree::MemTree;
use self::teststore::{TestLink, TestStore};

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
        |_store, _plan, attestation| async move {
            assert_eq!(attestation.plan, attestation.output);
            Ok(())
        },
    )
    .await
}

#[tokio::test]
async fn identity() -> anyhow::Result<()> {
    verify_guests(&["identity"], b"", |_store, plan, attestation| async move {
        assert_eq!(plan.input, attestation.output);
        Ok(())
    })
    .await
}

#[tokio::test]
async fn input_is_hello_world() -> anyhow::Result<()> {
    verify_guests(
        &[
            "test_input_is_hello_world",
            "test_bindings_input_is_hello_world",
        ],
        b"Hello World!",
        |_, _, _| async { Ok(()) },
    )
    .await
}

#[tokio::test]
async fn output_is_hello_world() -> anyhow::Result<()> {
    verify_guests(
        &["test_output_is_hello_world"],
        b"",
        |store, _, attestation| async move {
            let output: Vec<u8> = store.load(&attestation.output).await?;
            assert_eq!(output, b"Hello World!");
            Ok(())
        },
    )
    .await
}

#[tokio::test]
async fn reverse_contents() -> anyhow::Result<()> {
    verify_guests(
        &["test_reverse_contents"],
        [
            ("alpha", MemTree::from(b"alpha file")),
            (
                "beta",
                MemTree::from([
                    ("fruit", MemTree::from(b"banana")),
                    ("creature", MemTree::from(b"barnacle")),
                ]),
            ),
        ],
        |store, _, attestation| async move {
            let output: MemTree = store.load(&attestation.output).await?;

            assert_eq!(
                output,
                MemTree::from([
                    ("ahpla", MemTree::from(b"elif ahpla")),
                    (
                        "ateb",
                        MemTree::from([
                            ("tiurf", MemTree::from(b"ananab")),
                            ("erutaerc", MemTree::from(b"elcanrab")),
                        ])
                    ),
                ])
            );

            Ok(())
        },
    )
    .await
}
async fn verify_guests<M, F, Fut>(guests: &[&str], content: M, verify: F) -> anyhow::Result<()>
where
    MemTree: From<M>,
    F: Fn(TestStore, Plan<TestLink>, Attestation<TestLink>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    pangalactic_log::test_init();
    let r = verify_guests_inner(guests, MemTree::from(content), verify).await;
    if let Some(e) = r.as_ref().err() {
        eprintln!("{e:#}");
    }
    r
}

async fn verify_guests_inner<F, Fut>(
    guests: &[&str],
    content: MemTree,
    verify: F,
) -> anyhow::Result<()>
where
    F: Fn(TestStore, Plan<TestLink>, Attestation<TestLink>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    for guest in guests {
        verify_guest_inner(guest, content.clone(), &verify).await?;
    }
    Ok(())
}

async fn verify_guest_inner<F, Fut>(guest: &str, content: MemTree, verify: F) -> anyhow::Result<()>
where
    F: Fn(TestStore, Plan<TestLink>, Attestation<TestLink>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let mut store = TestStore::default();

    // Set up plan:
    let exec = store
        .commit(pangalactic_seed::get_wasm_bytes(guest)?)
        .await?;
    let input = store.commit(content).await?;

    let plan = store.commit(Plan { exec, input }).await?;

    // Execute derive:
    let (store, attestation) = store.derive(&plan).await?;

    let att: Attestation<TestLink> = store.load(&attestation).await?;
    let plan: Plan<TestLink> = store.load(&att.plan).await?;

    // Verify
    verify(store, plan, att).await?;

    Ok(())
}
