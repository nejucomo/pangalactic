use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_layer_host::HostLayer;
use pangalactic_layer_path::StorePath;
use pangalactic_link::Link;
use pangalactic_schemata::{Attestation, Plan};
use pangalactic_store::Store;
use pangalactic_store_mem::MemStore;
use std::future::Future;

mod memtree;
use self::memtree::MemTree;

type TestLink = Link<CidMeta<<MemStore as Store>::CID>>;

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
            let output: Vec<u8> = store.storedir_ref().load(&attestation.output).await?;
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
            let output: MemTree = store.storedir_ref().load(&attestation.output).await?;

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
    F: Fn(HostLayer<MemStore>, Plan<TestLink>, Attestation<TestLink>) -> Fut,
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
    F: Fn(HostLayer<MemStore>, Plan<TestLink>, Attestation<TestLink>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    for guest in guests {
        verify_guest_inner(guest, content.clone(), &verify).await?;
    }
    Ok(())
}

async fn verify_guest_inner<F, Fut>(guest: &str, content: MemTree, verify: F) -> anyhow::Result<()>
where
    F: Fn(HostLayer<MemStore>, Plan<TestLink>, Attestation<TestLink>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let mut store = HostLayer::default();

    let plan = {
        // Set up plan:
        let dstore = store.storedir_mut();
        let exec = dstore
            .commit(pangalactic_guests::get_wasm_bytes(guest)?)
            .await?;
        let input = dstore.commit(content).await?;

        dstore.commit(Plan { exec, input }).await?
    };

    // Execute derive:
    let attestation = store
        .derive(StorePath::from(plan))
        .await?
        .unwrap_pathless_link()?;

    let att: Attestation<TestLink> = store.storedir_ref().load(&attestation).await?;
    let plan: Plan<TestLink> = store.storedir_mut().load(&att.plan).await?;

    // Verify
    verify(store, plan, att).await?;

    Ok(())
}
