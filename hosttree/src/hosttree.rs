use crate::TreeNode;
use async_trait::async_trait;
use not_empty::NonEmptySlice;
use pangalactic_dagio::{Dagio, FromDag, HostDirectoryFor, LinkFor, ToDag};
use pangalactic_dir::{Directory, Name};
use pangalactic_hostdir::HostDirectory;
use pangalactic_store::Store;

#[derive(Debug)]
pub struct HostTree<S>(Directory<TreeNode<S>>)
where
    S: Store;

impl<S> HostTree<S>
where
    S: Store,
{
    pub async fn read<T>(
        &mut self,
        dagio: &mut Dagio<S>,
        names: &NonEmptySlice<Name>,
    ) -> anyhow::Result<T>
    where
        T: FromDag<S>,
    {
        let (last, intermediates) = names.split_last();
        let mut ht = self;
        for (i, name) in intermediates.into_iter().enumerate() {
            let node = ht.0.get_mut(name).ok_or_else(|| {
                let path = intermediates[..=i].join("/");
                anyhow::anyhow!("{path} is unlinked")
            })?;
            ht = node.load_tree(dagio).await?;
        }
        let node = ht.0.get_mut(last).ok_or_else(|| {
            let path = intermediates.join("/");
            anyhow::anyhow!("{path} is unlinked")
        })?;
        let link = node.load_link(dagio).await?;
        dagio.read(link).await
    }
}

impl<S> Clone for HostTree<S>
where
    S: Store,
{
    fn clone(&self) -> Self {
        HostTree(self.0.clone())
    }
}

#[async_trait]
impl<S> ToDag<S> for HostTree<S>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        let mut d = HostDirectory::default();
        for (name, ht) in self.0 {
            let link = ht.into_dag(dagio).await?;
            d.insert(name, link)?;
        }
        d.into_dag(dagio).await
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
        Ok(HostTree(
            hd.into_iter()
                .map(|(name, link)| (name, TreeNode::from(link)))
                .collect(),
        ))
    }
}
