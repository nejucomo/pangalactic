use pangalactic_dir::Directory;
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_name::NameRef;
use pangalactic_store::{Commit, Load, Store};

use self::Revision::*;

/// A Successive content produced by a `PublishCap`
///
/// Should this be in pangalactic-schemata? Guests should not see revisions.
pub enum Revision<L> {
    Null,
    Initial { content: L },
    Subsequent { content: L, prev: L },
}

impl<L> From<Revision<L>> for Directory<L> {
    fn from(rev: Revision<L>) -> Self {
        let mut d = Directory::default();
        match rev {
            Null => {}
            Initial { content } => d.insert("content".to_string(), content).unwrap(),
            Subsequent { content, prev } => {
                d.insert("content".to_string(), content).unwrap();
                d.insert("prev".to_string(), prev).unwrap();
            }
        };

        d
    }
}

impl<L> TryFrom<Directory<L>> for Revision<L> {
    type Error = anyhow::Error;

    fn try_from(mut dir: Directory<L>) -> anyhow::Result<Self> {
        if let Some(content) = dir.remove(NameRef::from_static("content")) {
            if let Some(prev) = dir.remove(NameRef::from_static("prev")) {
                Ok(Subsequent { content, prev })
            } else {
                Ok(Initial { content })
            }
        } else {
            dir.require_empty()?;
            Ok(Null)
        }
    }
}

impl<S> Load<LinkDirectoryLayer<S>> for Revision<Link<S::CID>>
where
    S: Store,
{
    async fn load_from_store(
        store: &LinkDirectoryLayer<S>,
        link: &Link<S::CID>,
    ) -> anyhow::Result<Self> {
        let linkdir: LinkDirectory<_> = store.load(link).await?;
        Self::try_from(Directory::from(linkdir))
    }
}

impl<S> Commit<LinkDirectoryLayer<S>> for Revision<Link<S::CID>>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        store
            .commit(LinkDirectory::from(Directory::from(self)))
            .await
    }
}
