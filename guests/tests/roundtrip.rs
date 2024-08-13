use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_layer_host::HostLayer;
use pangalactic_layer_path::StorePath;
use pangalactic_link::Link;
use pangalactic_schemata::{Attestation, Plan};
use pangalactic_store::Store;
use pangalactic_store_mem::MemStore;

mod memtree;
use self::memtree::MemTree;

type TestLink = Link<CidMeta<<MemStore as Store>::CID>>;

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
    let mut store: HostLayer<MemStore> = HostLayer::default();
    let intree = MemTree::from(input);
    let expected = intree.clone();
    let link_in = store.storedir_mut().commit(intree).await?;
    let att_in = run_phase(&mut store, exec_in, link_in).await?;
    let att_out = run_phase(&mut store, exec_out, att_in.output).await?;
    let output: MemTree = store.storedir_ref().load(&att_out.output).await?;

    assert_eq!(output, expected);
    Ok(())
}

async fn run_phase(
    store: &mut HostLayer<MemStore>,
    execname: &str,
    input: TestLink,
) -> anyhow::Result<Attestation<TestLink>> {
    let plan = StorePath::from({
        let dstore = store.storedir_mut();
        let exec = dstore
            .commit(pangalactic_guests::get_wasm_bytes(execname)?)
            .await?;
        dstore.commit(Plan { exec, input }).await?
    });
    let attestation = store.derive(plan).await?.unwrap_pathless_link()?;
    let att: Attestation<TestLink> = store.storedir_ref().load(&attestation).await?;
    Ok(att)
}
