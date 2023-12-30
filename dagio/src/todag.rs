use crate::{Dagio, DagioLink};
use async_trait::async_trait;
use pangalactic_store::Store;

#[cfg_attr(not(doc), async_trait)]
pub trait DagioCommit<S>
where
    S: Store,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>>;
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioCommit<S> for DagioLink<S>
where
    S: Store,
    DagioLink<S>: Clone,
{
    async fn commit_into_dagio(self, _: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        Ok(self)
    }
}
