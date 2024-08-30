use std::{fmt::Debug, future::Future};

use anyhow::Result;
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

    fn into_source(
        self,
        _: &pangalactic_layer_dir::LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<L, B>>> + Send {
        std::future::ready(Ok(self))
    }
}
