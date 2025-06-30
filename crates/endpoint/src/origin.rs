use anyhow::Result;
use pangalactic_dag_transfer::{
    IntoSource,
    Source::{self, Branch, Leaf},
};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_store::Store;

use crate::{
    aliases::{OriginEndpointBranch, OriginEndpointLeaf},
    Endpoint::{MkHos, MkStdio},
    HostOrStore::{MkHost, MkStore},
    OriginEndpoint,
};

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
        tracing::debug!("loading origin {}", &self);
        self.map_io(|io| io.into_source_leaf())
            .map_host(|p| p.into_source(store))
            .map_store(|p| p.into_source(store))
            .await_futures()
            .await
            .transpose()
            .map(|seps| {
                seps.project_into(
                    |stdin| Leaf(MkStdio(stdin)),
                    |hostsrc| hostsrc.map_into(|l| Leaf(MkHos(MkHost(l))), |b| Branch(MkHost(b))),
                    |storesrc| {
                        storesrc.map_into(|l| Leaf(MkHos(MkStore(l))), |b| Branch(MkStore(b)))
                    },
                )
            })
    }
}
