use std::{fmt::Debug, future::Future};

use anyhow::Result;
use tokio::io::AsyncRead;

use crate::{BranchIter, IntoSource};

#[derive(Debug)]
pub enum Source<L, B> {
    Leaf(L),
    Branch(B),
}

impl<L, B> IntoSource for Source<L, B>
where
    L: Debug + Send + AsyncRead,
    B: Debug + Send + BranchIter,
{
    type Leaf = L;
    type Branch = B;

    fn into_source<S>(
        self,
        _: &pangalactic_layer_dir::LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<L, B>>> + Send
    where
        S: pangalactic_store::Store,
    {
        std::future::ready(Ok(self))
    }
}
