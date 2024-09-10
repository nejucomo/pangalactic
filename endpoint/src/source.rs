use anyhow::Result;
use pangalactic_dag_transfer::{
    IntoSource,
    Source::{self, Branch, Leaf},
};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_store::Store;

use crate::{
    aliases::{HostPathSource, LinkPathSource, OriginEndpointBranch, OriginEndpointLeaf},
    Endpoint::{self, MkHos, MkStdio},
    HostOrStore::{MkHost, MkStore},
    OriginEndpoint, Stdio,
};

pub(crate) async fn into_source_endpoints<S>(
    endpoint: OriginEndpoint<S::CID>,
    store: &LinkDirectoryLayer<S>,
) -> Result<Endpoint<Stdio, HostPathSource, LinkPathSource<S>>>
where
    S: Store,
{
    endpoint
        .map_io(|io| async move { Ok(io) })
        .map_host(|p| p.into_source(store))
        .map_store(|p| p.into_source(store))
        .await_futures()
        .await
        .transpose()
}

impl<S> IntoSource<S> for OriginEndpoint<S::CID>
where
    S: Store,
{
    type Leaf = OriginEndpointLeaf<S>;
    type Branch = OriginEndpointBranch<S>;

    async fn into_source(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> Result<Source<Self::Leaf, Self::Branch>> {
        let seps = into_source_endpoints(self, store).await?;
        Ok(seps.project_into(
            |stdin| Leaf(MkStdio(stdin)),
            |hostsrc| hostsrc.map_into(|l| Leaf(MkHos(MkHost(l))), |b| Branch(MkHost(b))),
            |storesrc| storesrc.map_into(|l| Leaf(MkHos(MkStore(l))), |b| Branch(MkStore(b))),
        ))
    }
}

#[cfg(test)]
mod tests;
