use crate::{HostTreeDestination, HostTreePath, TreeDir};
use async_trait::async_trait;
use not_empty::NonEmptySlice;
use pangalactic_dagio::{Dagio, FromDag, LinkFor, ToDag};
use pangalactic_dir::Name;
use pangalactic_store::Store;

#[derive(Debug)]
pub struct HostTree<S>(Inner<S>)
where
    S: Store;

impl<S> From<LinkFor<S>> for HostTree<S>
where
    S: Store,
{
    fn from(link: LinkFor<S>) -> Self {
        HostTree(Inner::from(link))
    }
}

#[derive(Debug, derive_more::From)]
enum Inner<S>
where
    S: Store,
{
    Linked(LinkFor<S>),
    Expanded(TreeDir<S>),
}
use Inner::*;

impl<S> HostTree<S>
where
    S: Store,
{
    pub async fn read_path<T>(dagio: &mut Dagio<S>, source: &HostTreePath<S>) -> anyhow::Result<T>
    where
        T: FromDag<S>,
    {
        let (link, names) = source.link_and_path_slice();
        let mut ht = HostTree::from(link);
        ht.read_relpath(dagio, names).await
    }

    pub async fn set_path<T>(
        dagio: &mut Dagio<S>,
        path: &HostTreeDestination<S>,
        object: T,
    ) -> anyhow::Result<LinkFor<S>>
    where
        T: ToDag<S>,
    {
        let (link, intermediates, last) = path.link_intermediates_and_last_name();

        // TODO: change `StoreDestination` to use `NonEmptyVec` to avoid this hack:
        let mut intercopy: Vec<Name> = intermediates.iter().map(|n| n.to_string()).collect();
        intercopy.push(last.to_string());
        let neslice = NonEmptySlice::new(&intercopy).unwrap();

        let mut ht = HostTree::from(link);
        ht.set_relpath(dagio, neslice, object).await?;
        dagio.commit(ht).await
    }

    pub async fn read_relpath<T>(
        &mut self,
        dagio: &mut Dagio<S>,
        relpath: &[Name],
    ) -> anyhow::Result<T>
    where
        T: FromDag<S>,
    {
        let subtree = self.subtree_mut(dagio, relpath).await?;
        let link = subtree.load_link(dagio).await?;
        dagio.read(&link).await
    }

    pub async fn set_relpath<T>(
        &mut self,
        dagio: &mut Dagio<S>,
        relpath: &NonEmptySlice<Name>,
        object: T,
    ) -> anyhow::Result<()>
    where
        T: ToDag<S>,
    {
        let link = dagio.commit(object).await?;
        let (last, prefix) = relpath.split_last();
        let subtree = self.subtree_mut(dagio, prefix).await?;
        let treedir = subtree.load_treedir(dagio).await?;
        treedir.insert(last.to_string(), HostTree::from(link))?;
        Ok(())
    }

    async fn load_treedir(&mut self, dagio: &mut Dagio<S>) -> anyhow::Result<&mut TreeDir<S>> {
        if let Linked(link) = &mut self.0 {
            let ht = TreeDir::from_dag(dagio, link).await?;
            *self = HostTree(Expanded(ht));
        }
        match &mut self.0 {
            Linked(_) => panic!("`HostTree::load_treedir` internal invariant failed"),
            Expanded(ht) => Ok(ht),
        }
    }

    async fn load_link(&mut self, dagio: &mut Dagio<S>) -> anyhow::Result<&LinkFor<S>> {
        if let Expanded(ht) = &mut self.0 {
            // TODO: This API + `IntoDag` requires an inefficient clone:
            let link = ht.clone().into_dag(dagio).await?;
            *self = HostTree(Linked(link));
        }
        match &self.0 {
            Linked(link) => Ok(link),
            Expanded(_) => panic!("`HostTree::load_link` internal invariant failed"),
        }
    }

    async fn subtree_mut(
        &mut self,
        dagio: &mut Dagio<S>,
        relpath: &[Name],
    ) -> anyhow::Result<&mut HostTree<S>> {
        let mut node = self;

        for (i, name) in relpath.into_iter().enumerate() {
            let treedir = node.load_treedir(dagio).await?;
            node = treedir.get_mut(name).ok_or_else(|| {
                let path = relpath[..=i].join("/");
                anyhow::anyhow!("{path} is unlinked")
            })?;
        }

        Ok(node)
    }
}

impl<S> Clone for HostTree<S>
where
    S: Store,
{
    fn clone(&self) -> Self {
        HostTree(match &self.0 {
            Linked(x) => Linked(x.clone()),
            Expanded(x) => Expanded(x.clone()),
        })
    }
}

#[async_trait]
impl<S> ToDag<S> for HostTree<S>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        match self.0 {
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
        Ok(Self::from(link.clone()))
    }
}
