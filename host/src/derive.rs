use crate::{
    store::{HostLayer, HostLink},
    Host,
};
use pangalactic_store::Store;

pub async fn derive<S>(
    store: HostLayer<S>,
    plan: &HostLink<S::CID>,
) -> anyhow::Result<(HostLayer<S>, HostLink<S::CID>)>
where
    S: Store,
{
    let mut host = Host::new()?;
    tracing::debug!(?plan, "deriving");
    let (store, attestation) = host.execute(store, plan).await?;
    tracing::info!(?plan, ?attestation, "derived");
    Ok((store, attestation))
}
