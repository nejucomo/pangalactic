use std::future::Future;

use anyhow::Result;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_store::Store;

use crate::{Destination, IntoSource};

pub trait TransferLayerExt<S, I, D>
where
    S: Store,
    I: IntoSource<S>,
    D: Destination<S>,
{
    fn transfer(
        &mut self,
        source: I,
        destination: D,
    ) -> impl Future<Output = Result<D::CID>> + Send;
}

impl<S, I, D> TransferLayerExt<S, I, D> for LinkDirectoryLayer<S>
where
    S: Store,
    I: IntoSource<S>,
    D: Destination<S>,
{
    async fn transfer(&mut self, source: I, destination: D) -> Result<D::CID> {
        let s = source.into_source(self).await?;
        destination.sink(self, s).await
    }
}
