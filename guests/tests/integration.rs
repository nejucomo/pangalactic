use dagwasm_dagio::{Dagio, LinkFor, ToDag};
use dagwasm_dir::Directory;
use dagwasm_memstore::MemStore;
use dagwasm_schemata::{Attestation, Plan};
use dagwasm_store::Store;
use std::future::Future;

#[tokio::test]
async fn plan_is_dir() -> anyhow::Result<()> {
    verify_guests(
        &["test_plan_is_dir", "test_bindings_plan_is_dir"],
        CFile(b""),
        |_, _, _| async { Ok(()) },
    )
    .await
}

#[tokio::test]
async fn get_plan_outputs_plan() -> anyhow::Result<()> {
    verify_guests(
        &["get_plan"],
        CFile(b""),
        |_dagio, _plan, attestation| async move {
            assert_eq!(attestation.plan, attestation.output);
            Ok(())
        },
    )
    .await
}

#[tokio::test]
async fn identity() -> anyhow::Result<()> {
    verify_guests(
        &["identity"],
        CFile(b""),
        |_dagio, plan, attestation| async move {
            assert_eq!(plan.input, attestation.output);
            Ok(())
        },
    )
    .await
}

#[tokio::test]
async fn input_is_hello_world() -> anyhow::Result<()> {
    verify_guests(
        &[
            "test_input_is_hello_world",
            "test_bindings_input_is_hello_world",
        ],
        CFile(b"Hello World!"),
        |_, _, _| async { Ok(()) },
    )
    .await
}

#[tokio::test]
async fn output_is_hello_world() -> anyhow::Result<()> {
    verify_guests(
        &["test_output_is_hello_world"],
        CFile(b""),
        |mut dagio, _, attestation| async move {
            let output = dagio.read_file(&attestation.output).await?;
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
        CDir(&[
            ("alpha", CFile(b"alpha file")),
            (
                "beta",
                CDir(&[
                    ("fruit", CFile(b"banana")),
                    ("creature", CFile(b"barnacle")),
                ]),
            ),
        ]),
        |mut dagio, _, attestation| async move {
            let mut top: Directory<_> = dagio.read(&attestation.output).await?;

            let ahpla_link = top.remove_required("ahpla")?;
            let ahpla_contents = dagio.read_file(&ahpla_link).await?;
            assert_eq!(&ahpla_contents, b"elif ahpla");

            let ateb_link = top.remove_required("ateb")?;

            let mut ateb: Directory<_> = dagio.read(&ateb_link).await?;
            let tiurf_link = ateb.remove_required("tiurf")?;
            let tiurf_contents = dagio.read_file(&tiurf_link).await?;
            assert_eq!(&tiurf_contents, b"ananab");

            let erutaerc_link = ateb.remove_required("erutaerc")?;
            let erutaerc_contents = dagio.read_file(&erutaerc_link).await?;
            assert_eq!(&erutaerc_contents, b"elcanrab");

            ateb.require_empty()?;
            top.require_empty()?;
            Ok(())
        },
    )
    .await
}

#[derive(Clone, Debug)]
enum Content {
    CFile(&'static [u8]),
    CDir(&'static [(&'static str, Content)]),
}
use Content::*;

#[async_trait::async_trait]
impl<S> ToDag<S> for Content
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        match self {
            CFile(bytes) => dagio.write_file(bytes).await,
            CDir(entries) => {
                let mut d = Directory::default();
                for (n, child) in entries {
                    let link = child.clone().into_dag(dagio).await?;
                    d.insert(n.to_string(), link)?;
                }
                d.into_dag(dagio).await
            }
        }
    }
}

async fn verify_guests<F, Fut>(guests: &[&str], content: Content, verify: F) -> anyhow::Result<()>
where
    F: Fn(Dagio<MemStore>, Plan<LinkFor<MemStore>>, Attestation<LinkFor<MemStore>>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    dagwasm_log::test_init();
    let r = verify_guests_inner(guests, content, verify).await;
    if let Some(e) = r.as_ref().err() {
        eprintln!("{e:#}");
    }
    r
}

async fn verify_guests_inner<F, Fut>(
    guests: &[&str],
    content: Content,
    verify: F,
) -> anyhow::Result<()>
where
    F: Fn(Dagio<MemStore>, Plan<LinkFor<MemStore>>, Attestation<LinkFor<MemStore>>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    for guest in guests {
        verify_guest_inner(guest, content.clone(), &verify).await?;
    }
    Ok(())
}

async fn verify_guest_inner<F, Fut>(guest: &str, content: Content, verify: F) -> anyhow::Result<()>
where
    F: Fn(Dagio<MemStore>, Plan<LinkFor<MemStore>>, Attestation<LinkFor<MemStore>>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let mut dagio = Dagio::from(MemStore::default());

    let plan = {
        // Set up plan:
        let exec = dagio
            .write_file(dagwasm_guests::get_wasm_bytes(guest)?)
            .await?;
        let input = dagio.commit(content).await?;

        dagio.commit(Plan { exec, input }).await?
    };

    // Execute derive:
    let (mut dagio, attestation) = dagwasm_host::derive(dagio, &plan).await?;

    let att: Attestation<LinkFor<MemStore>> = dagio.read(&attestation).await?;
    let plan: Plan<LinkFor<MemStore>> = dagio.read(&att.plan).await?;

    // Verify
    verify(dagio, plan, att).await?;

    Ok(())
}
