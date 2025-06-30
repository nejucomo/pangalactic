use anyhow::Result;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::Link;
use pangalactic_store::{Commit, Store};

use crate::NestedDirectory;

#[derive(Clone, Debug)]
pub enum NDBranch<N, L = ()> {
    Subdir(Box<NestedDirectory<N, L>>),
    Leaf(L),
}
use NDBranch::*;

impl<S, L> Commit<LinkDirectoryLayer<S>> for NDBranch<(), L>
where
    S: Store,
    L: Commit<LinkDirectoryLayer<S>> + Send,
{
    async fn commit_into_store(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>> {
        match self {
            Subdir(bnd) => store.commit(bnd).await,
            Leaf(leaf) => store.commit(leaf).await,
        }
    }
}
