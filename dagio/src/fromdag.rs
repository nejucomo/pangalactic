use crate::{Dagio, DagioLink};
use async_trait::async_trait;
use pangalactic_store::Store;

#[cfg_attr(not(doc), async_trait)]
pub trait DagioLoad<S>: Sized
where
    S: Store,
{
    async fn load_from_dagio(dagio: &mut Dagio<S>, link: &DagioLink<S>) -> anyhow::Result<Self>;
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioLoad<S> for DagioLink<S>
where
    S: Store,
    DagioLink<S>: Clone,
{
    async fn load_from_dagio(_: &mut Dagio<S>, link: &DagioLink<S>) -> anyhow::Result<Self> {
        Ok(link.clone())
    }
}
