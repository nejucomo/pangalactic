use anyhow::Result;
use pangalactic_path::{AnyDestination, AnySource, PathLayer, StorePath};
use pangalactic_store::Store;

use crate::TransferInto;

pub trait Transferor<C> {
    async fn transfer(
        &mut self,
        source: AnySource<C>,
        destination: AnyDestination<C>,
    ) -> Result<Option<StorePath<C>>>;
}

impl<S> Transferor<S::CID> for PathLayer<S>
where
    S: Store,
    AnySource<S::CID>: TransferInto<S, AnyDestination<S::CID>>,
{
    async fn transfer(
        &mut self,
        source: AnySource<S::CID>,
        destination: AnyDestination<S::CID>,
    ) -> Result<Option<StorePath<S::CID>>> {
        source.transfer_into(self, destination).await
    }
}
