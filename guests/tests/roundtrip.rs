use dagwasm_dagio::{Dagio, LinkFor};
use dagwasm_memstore::MemStore;
use dagwasm_schemata::{Attestation, Plan};

#[tokio::test]
async fn gzip_gunzip_round_trip() -> anyhow::Result<()> {
    const INPUT: &[u8] = b"Hello World!";

    dagwasm_log::test_init();

    let mut dagio = Dagio::from(MemStore::default());

    let input = dagio.write_file(INPUT).await?;
    let (dagio, att_gzip) = run_phase(dagio, "gzip", input).await?;
    let (mut dagio, att_gunzip) = run_phase(dagio, "gunzip", att_gzip.output).await?;
    let outbytes = dagio.read_file(&att_gunzip.output).await?;

    assert_eq!(outbytes, INPUT);
    Ok(())
}

async fn run_phase(
    mut dagio: Dagio<MemStore>,
    execname: &str,
    input: LinkFor<MemStore>,
) -> anyhow::Result<(Dagio<MemStore>, Attestation<LinkFor<MemStore>>)> {
    let exec = dagio
        .write_file(dagwasm_guests::get_wasm_bytes(execname)?)
        .await?;
    let plan = dagio.commit(Plan { exec, input }).await?;
    let (mut dagio, attestation) = dagwasm_host::derive(dagio, &plan).await?;
    let att: Attestation<LinkFor<MemStore>> = dagio.read(&attestation).await?;
    Ok((dagio, att))
}
