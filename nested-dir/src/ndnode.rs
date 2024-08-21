use anyhow::Result;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::Link;
use pangalactic_store::{Commit, Store};

use crate::NestedDirectory;

#[derive(Debug)]
pub enum NDNode<L, B = ()> {
    Branch(Box<NestedDirectory<L, B>>, B),
    Leaf(L),
}
use NDNode::*;

impl<S, L> Commit<LinkDirectoryLayer<S>> for NDNode<L>
where
    S: Store,
    L: Commit<LinkDirectoryLayer<S>> + Send,
{
    async fn commit_into_store(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>> {
        match self {
            Branch(bnd, ()) => store.commit(bnd).await,
            Leaf(leaf) => store.commit(leaf).await,
        }
    }
}
