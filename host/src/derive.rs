use crate::Host;
use pangalactic_dagio::{Dagio, DagioLink};
use pangalactic_store::Store;

pub async fn derive<S>(
    dagio: Dagio<S>,
    plan: &DagioLink<S>,
) -> anyhow::Result<(Dagio<S>, DagioLink<S>)>
where
    S: Store,
{
    let mut host = Host::new()?;
    log::debug!("deriving {:?}...", plan);
    let (dagio, attestation) = host.execute(dagio, plan).await?;
    log::info!("attestation of {:?} -> {:?}", plan, &attestation);
    Ok((dagio, attestation))
}
