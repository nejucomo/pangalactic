use std::fmt::Debug;

use anyhow::Result;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_store::Store;
use tokio::io::AsyncRead;

use crate::{BranchIter, IntoSource};

#[derive(Debug)]
pub enum Source<L, B> {
    Leaf(L),
    Branch(B),
}

impl<S, L, B> IntoSource<S> for Source<L, B>
where
    S: Store,
    L: Debug + Send + AsyncRead,
    B: Debug + Send + BranchIter<S>,
{
    type Leaf = L;
    type Branch = B;

    async fn into_source(self, _: &LinkDirectoryLayer<S>) -> Result<Source<L, B>> {
        Ok(self)
    }
}
