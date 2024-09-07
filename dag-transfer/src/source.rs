use std::fmt::Debug;

use anyhow::Result;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_store::Store;
use tokio::io::AsyncRead;

use crate::{BranchIter, IntoSource};

use self::Source::*;

#[derive(Debug)]
pub enum Source<L, B> {
    Leaf(L),
    Branch(B),
}

impl<L, B> Source<L, B> {
    pub fn map_leaf<F, LR>(self, f: F) -> Source<LR, B>
    where
        F: FnOnce(L) -> LR,
    {
        match self {
            Leaf(l) => Leaf(f(l)),
            Branch(b) => Branch(b),
        }
    }

    pub fn map_branch<F, BR>(self, f: F) -> Source<L, BR>
    where
        F: FnOnce(B) -> BR,
    {
        match self {
            Leaf(l) => Leaf(l),
            Branch(b) => Branch(f(b)),
        }
    }

    pub fn map_into<F, G, T>(self, leaf_into: F, branch_into: G) -> T
    where
        F: FnOnce(L) -> T,
        G: FnOnce(B) -> T,
    {
        match self {
            Leaf(l) => leaf_into(l),
            Branch(b) => branch_into(b),
        }
    }
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
