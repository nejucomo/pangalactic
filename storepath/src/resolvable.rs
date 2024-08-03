use std::borrow::Borrow;

use async_trait::async_trait;
use pangalactic_hostdir::{HostDirectory, HostDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::Store;

use crate::StorePath;

#[cfg_attr(not(doc), async_trait)]
pub trait LinkResolvable<'a, S>
where
    S: Store,
{
    type Proxy: Borrow<Link<S::CID>>;

    async fn resolve_link(&'a self, store: &HostDirectoryLayer<S>) -> anyhow::Result<Self::Proxy>;
}

#[cfg_attr(not(doc), async_trait)]
impl<'a, S> LinkResolvable<'a, S> for Link<S::CID>
where
    S: Store,
    S::CID: 'a,
{
    type Proxy = &'a Self;

    async fn resolve_link(&'a self, _: &HostDirectoryLayer<S>) -> anyhow::Result<&'a Self> {
        Ok(self)
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<'a, S> LinkResolvable<'a, S> for &'a StorePath<S::CID>
where
    S: Store,
{
    type Proxy = Link<S::CID>;

    async fn resolve_link(&'a self, store: &HostDirectoryLayer<S>) -> anyhow::Result<Link<S::CID>> {
        let mut link = self.link().clone();
        for name in self.path() {
            let mut d: HostDirectory<S::CID> = store.load(&link).await?;
            link = d.remove_required(name)?;
        }
        Ok(link)
    }
}
