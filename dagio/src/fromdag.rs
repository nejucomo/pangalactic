use crate::{Dagio, LinkFor};
use async_trait::async_trait;
use dagwasm_store::Store;

#[async_trait]
pub trait FromDag<B>: Sized
where
    B: Store,
{
    async fn from_dag(dagio: &mut Dagio<B>, link: &LinkFor<B>) -> anyhow::Result<Self>;
}

#[async_trait]
impl<B> FromDag<B> for LinkFor<B>
where
    B: Store,
    LinkFor<B>: Clone,
{
    async fn from_dag(_: &mut Dagio<B>, link: &LinkFor<B>) -> anyhow::Result<Self> {
        Ok(link.clone())
    }
}
