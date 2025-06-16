use anyhow::Result;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::Link;
use pangalactic_name::Name;
use pangalactic_store::{Commit, Store};

use crate::{DfsIter, NDBranch};

#[derive(Clone, Debug)]
pub struct NDNode<N, L = ()> {
    pub data: N,
    pub branch: NDBranch<N, L>,
}

impl<N, L> NDNode<N, L> {
    pub fn into_depth_first_iter(self) -> impl Iterator<Item = (Vec<Name>, N, Option<L>)> {
        DfsIter::from(self)
    }
}

impl<S, L> Commit<LinkDirectoryLayer<S>> for NDNode<(), L>
where
    S: Store,
    L: Commit<LinkDirectoryLayer<S>> + Send,
{
    async fn commit_into_store(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>> {
        self.branch.commit_into_store(store).await
    }
}
