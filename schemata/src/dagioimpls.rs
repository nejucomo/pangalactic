use crate::{Attestation, Plan};
use async_trait::async_trait;
use dagwasm_dagio::{Dagio, FromDag, LinkFor, ToDag};
use dagwasm_dir::Directory;
use dagwasm_store::Store;

#[async_trait]
impl<S> FromDag<S> for Attestation<LinkFor<S>>
where
    S: Store,
{
    async fn from_dag(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        let dir = Directory::from_dag(dagio, link).await?;
        Self::try_from(dir)
    }
}

#[async_trait]
impl<S> ToDag<S> for Attestation<LinkFor<S>>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        dagio.commit(Directory::from(self)).await
    }
}

#[async_trait]
impl<S> FromDag<S> for Plan<LinkFor<S>>
where
    S: Store,
{
    async fn from_dag(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        let dir = Directory::from_dag(dagio, link).await?;
        Self::try_from(dir)
    }
}

#[async_trait]
impl<S> ToDag<S> for Plan<LinkFor<S>>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        dagio.commit(Directory::from(self)).await
    }
}
