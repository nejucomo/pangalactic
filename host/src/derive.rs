use crate::Host;
use pangalactic_dagio::{Dagio, LinkFor};
use pangalactic_store::Store;

pub async fn derive<S>(dagio: Dagio<S>, plan: &LinkFor<S>) -> anyhow::Result<(Dagio<S>, LinkFor<S>)>
where
    S: Store,
{
    let mut host = Host::new()?;
    log::debug!("deriving {:?}...", plan);
    let (dagio, attestation) = host.execute(dagio, plan).await?;
    log::info!("attestation of {:?} -> {:?}", plan, &attestation);
    Ok((dagio, attestation))
}
