use crate::HostTree;
use async_trait::async_trait;
use derive_more::{Deref, DerefMut, From};
use pangalactic_dagio::{Dagio, FromDag, LinkFor, ToDag};
use pangalactic_dir::Directory;
use pangalactic_hostdir::HostDirectory;
use pangalactic_store::Store;

#[derive(Debug, Deref, DerefMut, From)]
pub(crate) struct TreeDir<S>(Directory<HostTree<S>>)
where
    S: Store;

impl<S> Clone for TreeDir<S>
where
    S: Store,
{
    fn clone(&self) -> Self {
        TreeDir(self.0.clone())
    }
}

#[async_trait]
impl<S> ToDag<S> for TreeDir<S>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        let mut d = HostDirectory::default();
        for (name, ht) in self.0 {
            let link: LinkFor<S> = dagio.commit(ht).await?;
            d.insert(name, link)?;
        }
        dagio.commit(d).await
    }
}

#[async_trait]
impl<S> FromDag<S> for TreeDir<S>
where
    S: Store,
{
    async fn from_dag(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        let d = HostDirectory::from_dag(dagio, link).await?;
        Ok(TreeDir(
            d.into_iter()
                .map(|(name, link)| (name, HostTree::from(link)))
                .collect(),
        ))
    }
}
