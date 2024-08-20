use pangalactic_host::HostLayerExt;
use pangalactic_schemata::{Attestation, Plan};
use pangalactic_store::Store;

mod memtree;
mod teststore;

use self::memtree::MemTree;
use self::teststore::{TestLink, TestStore};

#[tokio::test]
async fn gzip_gunzip() -> anyhow::Result<()> {
    pangalactic_log::test_init();
    run_round_trip("gzip", "gunzip", b"Hello World!").await
}

#[tokio::test]
async fn tar_untar() -> anyhow::Result<()> {
    pangalactic_log::test_init();
    run_round_trip(
        "create_tar",
        "untar",
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
    )
    .await
}

async fn run_round_trip<M>(exec_in: &str, exec_out: &str, input: M) -> anyhow::Result<()>
where
    MemTree: From<M>,
{
    let mut store: TestStore = TestStore::default();
    let intree = MemTree::from(input);
    let expected = intree.clone();
    let link_in = store.commit(intree).await?;
    let (store, att_in) = run_phase(store, exec_in, link_in).await?;
    let (store, att_out) = run_phase(store, exec_out, att_in.output).await?;
    let output: MemTree = store.load(&att_out.output).await?;

    assert_eq!(output, expected);
    Ok(())
}

async fn run_phase(
    mut store: TestStore,
    execname: &str,
    input: TestLink,
) -> anyhow::Result<(TestStore, Attestation<TestLink>)> {
    let exec = store
        .commit(pangalactic_seed::get_wasm_bytes(execname)?)
        .await?;
    let plan = store.commit(Plan { exec, input }).await?;
    let (store, attestation) = store.derive(&plan).await?;
    let att: Attestation<TestLink> = store.load(&attestation).await?;
    Ok((store, att))
}
