use crate::Host;
use pangalactic_dagio::Dagio;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store::Store;

pub async fn derive<S>(
    dagio: Dagio<S>,
    plan: &Link<CidMeta<S::CID>>,
) -> anyhow::Result<(Dagio<S>, Link<CidMeta<S::CID>>)>
where
    S: Store,
{
    let mut host = Host::new()?;
    log::debug!("deriving {:?}...", plan);
    let (dagio, attestation) = host.execute(dagio, plan).await?;
    log::info!("attestation of {:?} -> {:?}", plan, &attestation);
    Ok((dagio, attestation))
}
