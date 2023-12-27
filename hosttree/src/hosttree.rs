use async_trait::async_trait;
use pangalactic_dagio::{Dagio, FromDag, HostDirectoryFor, LinkFor, ToDag};
use pangalactic_dir::Directory;
use pangalactic_hostdir::HostDirectory;
use pangalactic_store::Store;

#[derive(Debug, derive_more::From)]
pub enum HostTree<S>
where
    S: Store,
{
    LinkNode(LinkFor<S>),
    TreeNode(Directory<HostTree<S>>),
}
use HostTree::*;

#[async_trait]
impl<S> ToDag<S> for HostTree<S>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        match self {
            LinkNode(link) => Ok(link),
            TreeNode(treedir) => {
                let mut d = HostDirectory::default();
                for (name, subtree) in treedir {
                    let link = subtree.into_dag(dagio).await?;
                    d.insert(name, link)?;
                }
                d.into_dag(dagio).await
            }
        }
    }
}

#[async_trait]
impl<S> FromDag<S> for HostTree<S>
where
    S: Store,
{
    async fn from_dag(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        // We parse a single layer, leaving children as links:
        let hd = HostDirectoryFor::from_dag(dagio, link).await?;
        Ok(TreeNode(
            hd.into_iter()
                .map(|(name, link)| (name, HostTree::from(link)))
                .collect(),
        ))
    }
}
