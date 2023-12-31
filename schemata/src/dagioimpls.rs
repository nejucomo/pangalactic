use crate::{Attestation, Plan};
use async_trait::async_trait;
use pangalactic_dagio::{Dagio, DagioCommit, DagioLink, DagioLoad};
use pangalactic_dir::Directory;
use pangalactic_hostdir::HostDirectory;
use pangalactic_store::Store;

#[async_trait]
impl<S> DagioLoad<S> for Attestation<DagioLink<S>>
where
    S: Store,
{
    async fn load_from_dagio(dagio: &mut Dagio<S>, link: &DagioLink<S>) -> anyhow::Result<Self> {
        let hostdir: HostDirectory<_> = dagio.load(link).await?;
        Self::try_from(Directory::from(hostdir))
    }
}

#[async_trait]
impl<S> DagioCommit<S> for Attestation<DagioLink<S>>
where
    S: Store,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        dagio
            .commit(HostDirectory::from(Directory::from(self)))
            .await
    }
}

#[async_trait]
impl<S> DagioLoad<S> for Plan<DagioLink<S>>
where
    S: Store,
{
    async fn load_from_dagio(dagio: &mut Dagio<S>, link: &DagioLink<S>) -> anyhow::Result<Self> {
        let hostdir: HostDirectory<_> = dagio.load(link).await?;
        Self::try_from(Directory::from(hostdir))
    }
}

#[async_trait]
impl<S> DagioCommit<S> for Plan<DagioLink<S>>
where
    S: Store,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        dagio
            .commit(HostDirectory::from(Directory::from(self)))
            .await
    }
}
