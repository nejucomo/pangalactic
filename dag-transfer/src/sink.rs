use std::future::Future;

use anyhow::Result;
use tokio::io::AsyncRead;

use crate::{BranchIter, BranchSource, LeafOrBranchSource, LeafSource, Source};

pub trait Sink<S>
where
    S: Source,
{
    type CID;

    fn sink(self, source: S) -> impl Future<Output = Result<Self::CID>>;
}

impl<L, B, S> Sink<LeafOrBranchSource<L, B>> for S
where
    L: AsyncRead + Send,
    B: BranchIter,
    S: Sink<LeafSource<L>> + Sink<BranchSource<B>, CID = <S as Sink<LeafSource<L>>>::CID>,
{
    type CID = <S as Sink<LeafSource<L>>>::CID;

    async fn sink(self, source: LeafOrBranchSource<L, B>) -> Result<Self::CID> {
        use LeafOrBranchSource::*;

        match source {
            Leaf(l) => self.sink(LeafSource(l)).await,
            Branch(b) => self.sink(BranchSource(b)).await,
        }
    }
}
