use crate::{Dagio, LinkFor};
use async_trait::async_trait;
use pangalactic_store::Store;

#[cfg_attr(not(doc), async_trait)]
pub trait ToDag<S>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>>;
}

#[cfg_attr(not(doc), async_trait)]
impl<S> ToDag<S> for LinkFor<S>
where
    S: Store,
    LinkFor<S>: Clone,
{
    async fn into_dag(self, _: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        Ok(self)
    }
}
