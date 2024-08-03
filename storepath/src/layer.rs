use async_trait::async_trait;
use pangalactic_hostdir::{HostDirectory, HostDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::{Commit, Load, Store};

use crate::{StorePath, ViaPath};

#[derive(Debug, Default, derive_more::From)]
pub struct PathLayer<S>(HostDirectoryLayer<S>)
where
    S: Store;

impl<S> PathLayer<S>
where
    S: Store,
{
    pub(crate) async fn resolve_path(&self, p: &StorePath<S::CID>) -> anyhow::Result<Link<S::CID>> {
        let mut link = p.link().clone();
        for name in p.path() {
            let mut d: HostDirectory<S::CID> = self.0.load(&link).await?;
            link = d.remove_required(name)?;
        }
        Ok(link)
    }
}

#[async_trait]
impl<S> Store for PathLayer<S>
where
    S: Store,
{
    type CID = StorePath<S::CID>;
    type Reader = ViaPath<<HostDirectoryLayer<S> as Store>::Reader>;
    type Writer = ViaPath<<HostDirectoryLayer<S> as Store>::Writer>;

    async fn open_writer(&self) -> anyhow::Result<Self::Writer> {
        self.0.open_writer().await.map(ViaPath)
    }
}

#[async_trait]
impl<S, T> Commit<PathLayer<S>> for ViaPath<T>
where
    S: Store,
    T: Commit<HostDirectoryLayer<S>> + Send,
{
    async fn commit_into_store(
        self,
        store: &mut PathLayer<S>,
    ) -> anyhow::Result<StorePath<S::CID>> {
        let link = store.0.commit(self.0).await?;
        Ok(StorePath::from(link))
    }
}

#[async_trait]
impl<S, T> Load<PathLayer<S>> for ViaPath<T>
where
    S: Store,
    T: Load<HostDirectoryLayer<S>>,
{
    async fn load_from_store(
        store: &PathLayer<S>,
        path: &StorePath<S::CID>,
    ) -> anyhow::Result<Self> {
        let link = store.resolve_path(path).await?;
        let inner = store.0.load(&link).await?;
        Ok(ViaPath(inner))
    }
}
