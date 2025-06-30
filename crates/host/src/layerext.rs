use std::future::Future;

use anyhow::Result;
use extend::ext;
use pangalactic_layer_cidmeta::{CidMeta, CidMetaLayer};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::Link;
use pangalactic_store::Store;

use crate::Host;

#[ext(name = HostLayerExt, supertraits = Sized)]
pub impl<S> LinkDirectoryLayer<CidMetaLayer<S>>
where
    S: Store,
{
    #[allow(clippy::manual_async_fn)]
    fn derive(
        self,
        plan: &Link<CidMeta<S::CID>>,
    ) -> impl Future<Output = Result<(Self, Link<CidMeta<S::CID>>)>> + Send {
        async move {
            let mut host = Host::new()?;
            tracing::debug!(?plan, "deriving");
            let (newself, attestation) = host.execute(self, plan).await?;
            tracing::info!(?plan, ?attestation, "derived");
            Ok((newself, attestation))
        }
    }
}
