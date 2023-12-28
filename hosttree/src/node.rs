use crate::HostTree;
use async_trait::async_trait;
use pangalactic_dagio::{Dagio, FromDag, LinkFor, ToDag};
use pangalactic_store::Store;

#[derive(Debug, derive_more::From)]
pub enum TreeNode<S>
where
    S: Store,
{
    Linked(LinkFor<S>),
    Expanded(HostTree<S>),
}
use TreeNode::*;

impl<S> TreeNode<S>
where
    S: Store,
{
    pub(crate) async fn load_tree(
        &mut self,
        dagio: &mut Dagio<S>,
    ) -> anyhow::Result<&mut HostTree<S>> {
        if let Linked(link) = self {
            let ht = HostTree::from_dag(dagio, link).await?;
            *self = Expanded(ht);
        }
        match self {
            Linked(_) => panic!("`TreeNode::load_tree` internal invariant failed"),
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
            Expanded(_) => panic!("`TreeNode::load_link` internal invariant failed"),
        }
    }
}

impl<S> Clone for TreeNode<S>
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
impl<S> ToDag<S> for TreeNode<S>
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
impl<S> FromDag<S> for TreeNode<S>
where
    S: Store,
{
    async fn from_dag(_: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        Ok(Linked(link.clone()))
    }
}
