use crate::{Attestation, Plan};
use async_trait::async_trait;
use pangalactic_dagio::{Dagio, DagioCommit, DagioLoad, LinkFor};
use pangalactic_dir::Directory;
use pangalactic_hostdir::HostDirectory;
use pangalactic_store::Store;

#[async_trait]
impl<S> DagioLoad<S> for Attestation<LinkFor<S>>
where
    S: Store,
{
    async fn load_from_dagio(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        let hostdir = HostDirectory::load_from_dagio(dagio, link).await?;
        Self::try_from(Directory::from(hostdir))
    }
}

#[async_trait]
impl<S> DagioCommit<S> for Attestation<LinkFor<S>>
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
impl<S> DagioLoad<S> for Plan<LinkFor<S>>
where
    S: Store,
{
    async fn load_from_dagio(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        let hostdir = HostDirectory::load_from_dagio(dagio, link).await?;
        Self::try_from(Directory::from(hostdir))
    }
}

#[async_trait]
impl<S> DagioCommit<S> for Plan<LinkFor<S>>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        dagio
            .commit(HostDirectory::from(Directory::from(self)))
            .await
    }
}
