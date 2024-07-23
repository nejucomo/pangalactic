use std::borrow::Borrow;

use async_trait::async_trait;
use pangalactic_store::Store;

use crate::{Dagio, DagioHostDirectory, DagioLink, DagioStorePath};

#[cfg_attr(not(doc), async_trait)]
pub trait DagioResolveLink<S>
where
    S: Store,
{
    type Proxy: Borrow<DagioLink<S>>;

    async fn resolve_link(self, dagio: &Dagio<S>) -> anyhow::Result<Self::Proxy>;
}

#[cfg_attr(not(doc), async_trait)]
impl<'a, S> DagioResolveLink<S> for &'a DagioLink<S>
where
    S: Store,
{
    type Proxy = Self;

    async fn resolve_link(self, _: &Dagio<S>) -> anyhow::Result<Self> {
        Ok(self)
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioResolveLink<S> for DagioLink<S>
where
    S: Store,
{
    type Proxy = Self;

    async fn resolve_link(self, _: &Dagio<S>) -> anyhow::Result<Self> {
        Ok(self)
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<'a, S> DagioResolveLink<S> for &'a DagioStorePath<S>
where
    S: Store,
{
    type Proxy = DagioLink<S>;

    async fn resolve_link(self, dagio: &Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        let mut link = self.link().clone();
        for name in self.path() {
            let mut d: DagioHostDirectory<S> = dagio.load(link).await?;
            link = d.remove_required(name)?;
        }
        Ok(link)
    }
}
