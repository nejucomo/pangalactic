//! Forward implementations of [Commit] and [Load] from [HostDirectoryLayer](pangalactic_hostdir::HostDirectoryLayer) for convenience (all via [ViaPath])

use async_trait::async_trait;
use pangalactic_hostdir::{DirNodeReader, HostDirectory};
use pangalactic_store::{Commit, Load, Store};

use crate::{PathLayer, StorePath, ViaPath};

#[async_trait]
impl<S> Commit<PathLayer<S>> for HostDirectory<S::CID>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut PathLayer<S>,
    ) -> anyhow::Result<StorePath<S::CID>> {
        store.commit(ViaPath(self)).await
    }
}

#[async_trait]
impl<S> Load<PathLayer<S>> for HostDirectory<S::CID>
where
    S: Store,
{
    async fn load_from_store(
        store: &PathLayer<S>,
        path: &StorePath<S::CID>,
    ) -> anyhow::Result<Self> {
        let ViaPath(d) = store.load(path).await?;
        Ok(d)
    }
}

#[async_trait]
impl<S> Load<PathLayer<S>> for DirNodeReader<S>
where
    S: Store,
{
    async fn load_from_store(
        store: &PathLayer<S>,
        path: &StorePath<S::CID>,
    ) -> anyhow::Result<Self> {
        let ViaPath(d) = store.load(path).await?;
        Ok(d)
    }
}
