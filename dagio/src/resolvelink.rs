use std::borrow::Borrow;

use async_trait::async_trait;
use pangalactic_hostdir::HostDirectory;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store::Store;
use pangalactic_path::StorePath;

use crate::Dagio;

#[cfg_attr(not(doc), async_trait)]
pub trait DagioResolveLink<S>
where
    S: Store,
{
    type Proxy: Borrow<Link<CidMeta<S::CID>>>;

    async fn resolve_link(self, dagio: &Dagio<S>) -> anyhow::Result<Self::Proxy>;
}

#[cfg_attr(not(doc), async_trait)]
impl<'a, S> DagioResolveLink<S> for &'a Link<CidMeta<S::CID>>
where
    S: Store,
{
    type Proxy = Self;

    async fn resolve_link(self, _: &Dagio<S>) -> anyhow::Result<Self> {
        Ok(self)
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioResolveLink<S> for Link<CidMeta<S::CID>>
where
    S: Store,
{
    type Proxy = Self;

    async fn resolve_link(self, _: &Dagio<S>) -> anyhow::Result<Self> {
        Ok(self)
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<'a, S> DagioResolveLink<S> for &'a StorePath<CidMeta<S::CID>>
where
    S: Store,
{
    type Proxy = Link<CidMeta<S::CID>>;

    async fn resolve_link(self, dagio: &Dagio<S>) -> anyhow::Result<Link<CidMeta<S::CID>>> {
        let mut link = self.link().clone();
        for name in self.path() {
            let mut d: HostDirectory<S::CID> = dagio.load(link).await?;
            link = d.remove_required(name)?;
        }
        Ok(link)
    }
}
