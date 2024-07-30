use crate::{Attestation, Plan};
use async_trait::async_trait;
use pangalactic_dagio::{Dagio, DagioCommit, DagioLoad};
use pangalactic_dir::Directory;
use pangalactic_hostdir::HostDirectory;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store::Store;

#[async_trait]
impl<S> DagioLoad<S> for Attestation<Link<CidMeta<S::CID>>>
where
    S: Store,
{
    async fn load_from_dagio(
        dagio: &Dagio<S>,
        link: &Link<CidMeta<S::CID>>,
    ) -> anyhow::Result<Self> {
        let hostdir: HostDirectory<_> = dagio.load(link).await?;
        Self::try_from(Directory::from(hostdir))
    }
}

#[async_trait]
impl<S> DagioCommit<S> for Attestation<Link<CidMeta<S::CID>>>
where
    S: Store,
{
    async fn commit_into_dagio(
        self,
        dagio: &mut Dagio<S>,
    ) -> anyhow::Result<Link<CidMeta<S::CID>>> {
        dagio
            .commit(HostDirectory::from(Directory::from(self)))
            .await
    }
}

#[async_trait]
impl<S> DagioLoad<S> for Plan<Link<CidMeta<S::CID>>>
where
    S: Store,
{
    async fn load_from_dagio(
        dagio: &Dagio<S>,
        link: &Link<CidMeta<S::CID>>,
    ) -> anyhow::Result<Self> {
        let hostdir: HostDirectory<_> = dagio.load(link).await?;
        Self::try_from(Directory::from(hostdir))
    }
}

#[async_trait]
impl<S> DagioCommit<S> for Plan<Link<CidMeta<S::CID>>>
where
    S: Store,
{
    async fn commit_into_dagio(
        self,
        dagio: &mut Dagio<S>,
    ) -> anyhow::Result<Link<CidMeta<S::CID>>> {
        dagio
            .commit(HostDirectory::from(Directory::from(self)))
            .await
    }
}
