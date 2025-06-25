use std::fmt;

use anyhow::Result;
use pangalactic_dag_transfer::{BranchIter, Destination, LeafDestination};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_store::Store;
use tokio::io::AsyncRead;

use crate::{DestinationEndpoint, Receipt};

impl<S> Destination<S> for DestinationEndpoint<S::CID>
where
    S: Store,
{
    async fn sink_branch<B>(self, store: &mut LinkDirectoryLayer<S>, branch: B) -> Result<Self::CID>
    where
        B: fmt::Debug + Send + BranchIter<S>,
    {
        tracing::debug!("committing destination {}", &self);
        self.map_any_with(
            (store, branch),
            |io, (store, branch)| io.sink_branch(store, branch),
            |h, (store, branch)| h.sink_branch(store, branch),
            |s, (store, branch)| s.sink_branch(store, branch),
        )
        .await_futures()
        .await
        .transpose()
    }
}

impl<S> LeafDestination<S> for DestinationEndpoint<S::CID>
where
    S: Store,
{
    type CID = Receipt<S::CID>;

    async fn sink_leaf<L>(self, store: &mut LinkDirectoryLayer<S>, leaf: L) -> Result<Self::CID>
    where
        L: fmt::Debug + Send + AsyncRead,
    {
        tracing::debug!("committing destination {}", &self);
        self.map_any_with(
            (store, leaf),
            |io, (store, leaf)| io.sink_leaf(store, leaf),
            |h, (store, leaf)| h.sink_leaf(store, leaf),
            |s, (store, leaf)| s.sink_leaf(store, leaf),
        )
        .await_futures()
        .await
        .transpose()
    }
}
