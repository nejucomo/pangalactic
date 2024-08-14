use anyhow::Result;
use pangalactic_bindref::{BindRef, Bindable};
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::{Commit, Load, Store};

use crate::{AnyDestination, AnySource, StoreDestination, StorePath, ViaPath};

#[derive(Debug, Default, derive_more::From, derive_more::Into)]
pub struct PathLayer<S>(LinkDirectoryLayer<S>)
where
    S: Store;

impl<S> Bindable for PathLayer<S> where S: Store {}

impl<S> PathLayer<S>
where
    S: Store,
{
    pub async fn transfer(
        &mut self,
        source: AnySource<S::CID>,
        destination: AnyDestination<S::CID>,
    ) -> Result<Option<StorePath<S::CID>>> {
        use crate::transfer::TransferInto;

        source.transfer_into(self, destination).await
    }

    pub async fn resolve_path(&self, p: &StorePath<S::CID>) -> Result<Link<S::CID>> {
        let mut link = p.link().clone();
        for name in p.path() {
            let mut d: LinkDirectory<S::CID> = self.0.load(&link).await?;
            link = d.remove_required(name)?;
        }
        Ok(link)
    }
}

impl<S> AsRef<LinkDirectoryLayer<S>> for PathLayer<S>
where
    S: Store,
{
    fn as_ref(&self) -> &LinkDirectoryLayer<S> {
        &self.0
    }
}

impl<S> AsMut<LinkDirectoryLayer<S>> for PathLayer<S>
where
    S: Store,
{
    fn as_mut(&mut self) -> &mut LinkDirectoryLayer<S> {
        &mut self.0
    }
}

impl<S> Store for PathLayer<S>
where
    S: Store,
{
    type CID = StorePath<S::CID>;
    type Reader = ViaPath<<LinkDirectoryLayer<S> as Store>::Reader>;
    type Writer = ViaPath<<LinkDirectoryLayer<S> as Store>::Writer>;

    async fn open_writer(&self) -> Result<Self::Writer> {
        self.0.open_writer().await.map(ViaPath)
    }
}

impl<S, T> Commit<PathLayer<S>> for ViaPath<T>
where
    S: Store,
    T: Commit<LinkDirectoryLayer<S>> + Send,
{
    async fn commit_into_store(self, store: &mut PathLayer<S>) -> Result<StorePath<S::CID>> {
        let link = store.0.commit(self.0).await?;
        Ok(StorePath::from(link))
    }
}

impl<S, T> Load<PathLayer<S>> for ViaPath<T>
where
    S: Store,
    T: Load<LinkDirectoryLayer<S>>,
{
    async fn load_from_store(store: &PathLayer<S>, path: &StorePath<S::CID>) -> Result<Self> {
        let link = store.resolve_path(path).await?;
        let inner = store.0.load(&link).await?;
        Ok(ViaPath(inner))
    }
}

impl<'a, S, V> Commit<PathLayer<S>> for BindRef<'a, StoreDestination<S::CID>, V>
where
    S: Store,
    V: Commit<PathLayer<S>> + Send,
{
    async fn commit_into_store(self, store: &mut PathLayer<S>) -> Result<StorePath<S::CID>> {
        let BindRef { bound, value } = self;

        let valpath = store.commit(value).await?;
        let mut link = valpath.unwrap_pathless_link()?;

        let mut dirlink = StorePath::from(bound.link().clone());
        let mut stack = vec![];
        let (last, intermediate) = bound.path().split_last();

        for name in intermediate {
            let d: LinkDirectory<S::CID> = store.load(&dirlink).await?;
            dirlink = StorePath::from(d.get_required(name)?.clone());
            stack.push((d, name));
        }

        let mut d: LinkDirectory<S::CID> = store.load(&dirlink).await?;
        d.insert(last.clone(), link)?;

        for (mut prevd, name) in stack.into_iter().rev() {
            link = store.commit(d).await?.unwrap_pathless_link()?;
            prevd.overwrite(name.clone(), link);
            d = prevd;
        }

        store.commit(d).await
    }
}
