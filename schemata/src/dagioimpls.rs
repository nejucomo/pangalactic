use crate::{Attestation, Plan};
use async_trait::async_trait;
use pangalactic_dagio::{Dagio, FromDag, LinkFor, ToDag};
use pangalactic_dir::Directory;
use pangalactic_hostdir::HostDirectory;
use pangalactic_store::Store;

#[async_trait]
impl<S> FromDag<S> for Attestation<LinkFor<S>>
where
    S: Store,
{
    async fn from_dag(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        let hostdir = HostDirectory::from_dag(dagio, link).await?;
        Self::try_from(Directory::from(hostdir))
    }
}

#[async_trait]
impl<S> ToDag<S> for Attestation<LinkFor<S>>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        dagio
            .commit(HostDirectory::from(Directory::from(self)))
            .await
    }
}

#[async_trait]
impl<S> FromDag<S> for Plan<LinkFor<S>>
where
    S: Store,
{
    async fn from_dag(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        let hostdir = HostDirectory::from_dag(dagio, link).await?;
        Self::try_from(Directory::from(hostdir))
    }
}

#[async_trait]
impl<S> ToDag<S> for Plan<LinkFor<S>>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        dagio
            .commit(HostDirectory::from(Directory::from(self)))
            .await
    }
}
