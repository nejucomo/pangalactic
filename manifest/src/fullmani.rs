use anyhow::Result;
use derive_more::{Deref, DerefMut, From, Into};
use pangalactic_dir::Name;
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_nested_dir::NestedDirectory;
use pangalactic_store::{Load, Store};

use crate::DfsIter;

/// A Full manifest includes all links and all directory structure
#[derive(Debug, From, Into, Deref, DerefMut)]
pub struct FullManifest<C>(NestedDirectory<C, C>);

impl<C> Default for FullManifest<C> {
    fn default() -> Self {
        FullManifest(NestedDirectory::default())
    }
}

impl<C> IntoIterator for FullManifest<C> {
    type Item = (Vec<Name>, Link<C>);
    type IntoIter = DfsIter<C>;

    fn into_iter(self) -> Self::IntoIter {
        DfsIter::from(self.0.into_iter())
    }
}

impl<S> Load<LinkDirectoryLayer<S>> for FullManifest<S::CID>
where
    S: Store,
{
    async fn load_from_store(store: &LinkDirectoryLayer<S>, link: &Link<S::CID>) -> Result<Self> {
        use pangalactic_linkkind::LinkKind::*;
        use pangalactic_nested_dir::NDNode::*;

        let mut me = FullManifest::default();
        let ld: LinkDirectory<S::CID> = store.load(link).await?;
        for (name, link) in ld {
            let (kind, cid) = link.unwrap();
            let node = match kind {
                File => Leaf(cid),
                Dir => {
                    let FullManifest(nd) = store.load(&Link::new(kind, cid.clone())).await?;
                    Branch(Box::new(nd), cid)
                }
            };
            me.insert(name, node)?;
        }
        Ok(me)
    }
}
