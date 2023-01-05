use crate::Host;
use dagwasm_dagio::{Dagio, LinkFor};
use dagwasm_store::Store;
use std::ops::Deref;

pub async fn derive<S>(dagio: Dagio<S>, plan: &LinkFor<S>) -> anyhow::Result<(Dagio<S>, LinkFor<S>)>
where
    S: Store,
    <S as Store>::Writer: Deref,
    <<S as Store>::Writer as Deref>::Target: Unpin,
{
    let mut host = Host::new()?;
    log::debug!("deriving {:?}...", plan);
    let (dagio, attestation) = host.execute(dagio, plan).await?;
    log::info!("attestation of {:?} -> {:?}", plan, &attestation);
    Ok((dagio, attestation))
}
