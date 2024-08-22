use anyhow::Result;
use either::Either;
use pangalactic_dir::{Directory, Name};
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::{Commit, Store};

use crate::{DfsIter, NDNode};

#[derive(
    Debug, derive_more::Deref, derive_more::DerefMut, derive_more::From, derive_more::Into,
)]
pub struct NestedDirectory<L, B = ()>(Directory<NDNode<L, B>>);

impl<L, B> Default for NestedDirectory<L, B> {
    fn default() -> Self {
        NestedDirectory(Directory::default())
    }
}

impl<L> From<Directory<L>> for NestedDirectory<L> {
    fn from(d: Directory<L>) -> Self {
        NestedDirectory(d.map_values(NDNode::Leaf))
    }
}

impl<C> From<LinkDirectory<C>> for NestedDirectory<Link<C>> {
    fn from(ld: LinkDirectory<C>) -> Self {
        NestedDirectory::from(Directory::from(ld))
    }
}

impl<L, B> IntoIterator for NestedDirectory<L, B> {
    type Item = (Vec<Name>, Either<L, B>);
    type IntoIter = DfsIter<L, B>;

    fn into_iter(self) -> Self::IntoIter {
        DfsIter::from(self)
    }
}

impl<S, L> Commit<LinkDirectoryLayer<S>> for NestedDirectory<L>
where
    S: Store,
    L: Commit<LinkDirectoryLayer<S>> + Send,
{
    async fn commit_into_store(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>> {
        let mut d = LinkDirectory::default();
        for (name, node) in self.0 {
            let link = store.commit(node).await?;
            d.insert(name, link)?;
        }
        store.commit(d).await
    }
}
