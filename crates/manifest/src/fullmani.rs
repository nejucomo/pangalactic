use std::fmt::Display;

use anyhow::Result;
use derive_more::{Deref, DerefMut, From, Into};
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_name::Name;
use pangalactic_nested_dir::NestedDirectory;
use pangalactic_store::{Load, Store};

/// A Full manifest includes all links and all directory structure
#[derive(Debug, Clone, From, Into, Deref, DerefMut)]
pub struct FullManifest<C>(NestedDirectory<C, ()>);

impl<C> FullManifest<C> {
    pub fn into_depth_first_iter(self) -> impl Iterator<Item = (Vec<Name>, Link<C>)> {
        self.0.into_depth_first_iter().map(|(path, cid, optleaf)| {
            use pangalactic_linkkind::LinkKind::*;

            (
                path,
                Link::new(if optleaf.is_some() { File } else { Dir }, cid),
            )
        })
    }
}

impl<C> Default for FullManifest<C> {
    fn default() -> Self {
        FullManifest(NestedDirectory::default())
    }
}

impl<S> Load<LinkDirectoryLayer<S>> for FullManifest<S::CID>
where
    S: Store,
{
    async fn load_from_store(store: &LinkDirectoryLayer<S>, link: &Link<S::CID>) -> Result<Self> {
        use pangalactic_linkkind::LinkKind::*;
        use pangalactic_nested_dir::{NDBranch::*, NDNode};

        let mut fm = FullManifest::default();
        let ld: LinkDirectory<S::CID> = store.load(link).await?;
        for (name, link) in ld {
            let node = NDNode {
                data: link.peek_cid().clone(),
                branch: match link.kind() {
                    File => Leaf(()),
                    Dir => {
                        let FullManifest(nd) = Box::pin(store.load(&link)).await?;
                        Subdir(Box::new(nd))
                    }
                },
            };
            fm.insert(name, node)?;
        }
        Ok(fm)
    }
}

impl<C> Display for FullManifest<C>
where
    C: Clone,
    Link<C>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Replace this with a ref iter so we can avoid this clone:
        for (pathvec, link) in self.clone().into_depth_first_iter() {
            let path = pathvec.as_slice().join("/");
            writeln!(f, "{link} {path}")?;
        }
        Ok(())
    }
}
