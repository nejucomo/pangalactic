use crate::{Dagio, LinkFor};
use async_trait::async_trait;
use pangalactic_store::Store;

#[cfg_attr(not(doc), async_trait)]
pub trait FromDag<S>: Sized
where
    S: Store,
{
    async fn load_from_dagio(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self>;
}

#[cfg_attr(not(doc), async_trait)]
impl<S> FromDag<S> for LinkFor<S>
where
    S: Store,
    LinkFor<S>: Clone,
{
    async fn load_from_dagio(_: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        Ok(link.clone())
    }
}
