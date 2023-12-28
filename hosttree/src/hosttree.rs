use crate::{HostTreePath, TreeDir};
use async_trait::async_trait;
use not_empty::NonEmptySlice;
use pangalactic_dagio::{Dagio, FromDag, LinkFor, ToDag};
use pangalactic_dir::Name;
use pangalactic_store::Store;

#[derive(Debug, derive_more::From)]
pub enum HostTree<S>
where
    S: Store,
{
    Linked(LinkFor<S>),
    Expanded(TreeDir<S>),
}
use HostTree::*;

impl<S> HostTree<S>
where
    S: Store,
{
    pub async fn read_path<T>(dagio: &mut Dagio<S>, path: &HostTreePath<S>) -> anyhow::Result<T>
    where
        T: FromDag<S>,
    {
        let (link, names) = path.link_and_path_slice();
        let mut ht = HostTree::from(link);
        ht.read_relpath(dagio, names).await
    }

    pub async fn read_relpath<T>(
        &mut self,
        dagio: &mut Dagio<S>,
        relpath: &[Name],
    ) -> anyhow::Result<T>
    where
        T: FromDag<S>,
    {
        if let Some(names) = NonEmptySlice::new(relpath).ok() {
            let treedir = self.load_tree(dagio).await?;
            treedir.read(dagio, names).await
        } else {
            let link = self.load_link(dagio).await?;
            dagio.read(&link).await
        }
    }

    pub(crate) async fn load_tree(
        &mut self,
        dagio: &mut Dagio<S>,
    ) -> anyhow::Result<&mut TreeDir<S>> {
        if let Linked(link) = self {
            let ht = TreeDir::from_dag(dagio, link).await?;
            *self = Expanded(ht);
        }
        match self {
            Linked(_) => panic!("`HostTree::load_tree` internal invariant failed"),
            Expanded(ht) => Ok(ht),
        }
    }

    pub(crate) async fn load_link(&mut self, dagio: &mut Dagio<S>) -> anyhow::Result<&LinkFor<S>> {
        if let Expanded(ht) = self {
            // TODO: This API + `IntoDag` requires an inefficient clone:
            let link = ht.clone().into_dag(dagio).await?;
            *self = Linked(link);
        }
        match self {
            Linked(link) => Ok(link),
            Expanded(_) => panic!("`HostTree::load_link` internal invariant failed"),
        }
    }
}

impl<S> Clone for HostTree<S>
where
    S: Store,
{
    fn clone(&self) -> Self {
        match self {
            Linked(x) => Linked(x.clone()),
            Expanded(x) => Expanded(x.clone()),
        }
    }
}

#[async_trait]
impl<S> ToDag<S> for HostTree<S>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        match self {
            Linked(link) => Ok(link),
            Expanded(ht) => ht.into_dag(dagio).await,
        }
    }
}

#[async_trait]
impl<S> FromDag<S> for HostTree<S>
where
    S: Store,
{
    async fn from_dag(_: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        Ok(Linked(link.clone()))
    }
}
