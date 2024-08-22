use anyhow::Result;
use pangalactic_dir::{Directory, Name};
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::{Commit, Store};

use crate::NDNode;

#[derive(
    Debug, derive_more::Deref, derive_more::DerefMut, derive_more::From, derive_more::Into,
)]
pub struct NestedDirectory<N, L = ()>(Directory<NDNode<N, L>>);

impl<N, L> NestedDirectory<N, L> {
    pub fn into_depth_first_iter(self) -> impl Iterator<Item = (Vec<Name>, N, Option<L>)> {
        self.0.into_iter().flat_map(|(first_name, node)| {
            node.into_depth_first_iter()
                .map(move |(mut path, data, optleaf)| {
                    path.insert(0, first_name.clone());
                    (path, data, optleaf)
                })
        })
    }
}

impl<N, L> Default for NestedDirectory<N, L> {
    fn default() -> Self {
        NestedDirectory(Directory::default())
    }
}

impl<L> From<Directory<L>> for NestedDirectory<L> {
    fn from(d: Directory<L>) -> Self {
        NestedDirectory(d.map_values(|data| NDNode {
            data,
            branch: crate::NDBranch::Leaf(()),
        }))
    }
}

impl<C> From<LinkDirectory<C>> for NestedDirectory<Link<C>> {
    fn from(ld: LinkDirectory<C>) -> Self {
        NestedDirectory::from(Directory::from(ld))
    }
}

impl<N, L> From<NestedDirectory<N, L>>
    for std::collections::btree_map::IntoIter<Name, NDNode<N, L>>
{
    fn from(nd: NestedDirectory<N, L>) -> Self {
        nd.0.into_iter()
    }
}

impl<S, L> Commit<LinkDirectoryLayer<S>> for NestedDirectory<(), L>
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
