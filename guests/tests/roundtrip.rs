use pangalactic_dagio::{Dagio, LinkFor};
use pangalactic_memstore::MemStore;
use pangalactic_schemata::{Attestation, Plan};

mod memtree;
use self::memtree::MemTree;

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
    let intree = MemTree::from(input);
    let mut dagio: Dagio<MemStore> = Dagio::from(MemStore::default());
    let expected = intree.clone();
    let link_in = dagio.commit(intree).await?;
    let (dagio, att_in) = run_phase(dagio, exec_in, link_in).await?;
    let (mut dagio, att_out) = run_phase(dagio, exec_out, att_in.output).await?;
    let output: MemTree = dagio.read(&att_out.output).await?;

    assert_eq!(output, expected);
    Ok(())
}

async fn run_phase(
    mut dagio: Dagio<MemStore>,
    execname: &str,
    input: LinkFor<MemStore>,
) -> anyhow::Result<(Dagio<MemStore>, Attestation<LinkFor<MemStore>>)> {
    let exec = dagio
        .write_file(pangalactic_guests::get_wasm_bytes(execname)?)
        .await?;
    let plan = dagio.commit(Plan { exec, input }).await?;
    let (mut dagio, attestation) = pangalactic_host::derive(dagio, &plan).await?;
    let att: Attestation<LinkFor<MemStore>> = dagio.read(&attestation).await?;
    Ok((dagio, att))
}
