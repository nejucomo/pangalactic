use async_trait::async_trait;
use pangalactic_hostdir::HostDirectory;
use pangalactic_store::{Commit, Store};

use crate::{PathLayer, StoreDestination, StorePath, ViaPath};

#[derive(Debug)]
pub struct Destined<'a, C, T> {
    dest: &'a StoreDestination<C>,
    value: T,
}

impl<'a, C, T> Destined<'a, C, T> {
    pub(crate) fn new(dest: &'a StoreDestination<C>, value: T) -> Self {
        Destined { dest, value }
    }
}

#[async_trait]
impl<'a, S, T> Commit<PathLayer<S>> for Destined<'a, S::CID, T>
where
    S: Store,
    T: Commit<PathLayer<S>> + Send,
{
    async fn commit_into_store(
        self,
        store: &mut PathLayer<S>,
    ) -> anyhow::Result<StorePath<S::CID>> {
        let Destined { dest, value } = self;

        let valpath = store.commit(value).await?;
        let mut link = valpath.unwrap_pathless_link()?;

        let mut dirlink = StorePath::from(dest.link().clone());
        let mut stack = vec![];
        let (last, intermediate) = dest.path().split_last();

        for name in intermediate {
            let ViaPath(d): ViaPath<HostDirectory<S::CID>> = store.load(&dirlink).await?;
            dirlink = StorePath::from(d.get_required(name)?.clone());
            stack.push((d, name));
        }

        let ViaPath(mut d): ViaPath<HostDirectory<S::CID>> = store.load(&dirlink).await?;
        d.insert(last.clone(), link)?;

        for (mut prevd, name) in stack.into_iter().rev() {
            link = store.commit(ViaPath(d)).await?.unwrap_pathless_link()?;
            prevd.overwrite(name.clone(), link);
            d = prevd;
        }

        store.commit(ViaPath(d)).await
    }
}
