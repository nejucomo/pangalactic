use std::future::Future;

use anyhow::Result;
use pangalactic_asynctryiter::IntoAsyncTryIterator;
use pangalactic_name::Name;
use tokio::io::AsyncRead;

use crate::{BranchSource, LeafOrBranchSource, LeafSource, Source};

pub trait Sink<S>
where
    S: Source,
{
    type CID;

    fn sink(self, source: S) -> impl Future<Output = Result<Self::CID>>;
}

impl<R, I, S> Sink<LeafOrBranchSource<R, I>> for S
where
    R: AsyncRead + Send,
    I: IntoAsyncTryIterator<Item = (Name, LeafOrBranchSource<R, I>)> + Send,
    S: Sink<LeafSource<R>> + Sink<BranchSource<R, I>, CID = <S as Sink<LeafSource<R>>>::CID>,
{
    type CID = <S as Sink<LeafSource<R>>>::CID;

    async fn sink(self, source: LeafOrBranchSource<R, I>) -> Result<Self::CID> {
        use LeafOrBranchSource::*;

        match source {
            Leaf(l) => self.sink(LeafSource(l)).await,
            Branch(b) => self.sink(BranchSource(b)).await,
        }
    }
}
