//! Forward implementations of [Commit] and [Load] from [HostDirectoryLayer](pangalactic_hostdir::HostDirectoryLayer) for convenience (all via [ViaPath])

use async_trait::async_trait;
use pangalactic_store::{Commit, Load, Store};

use crate::{PathLayer, StorePath, ViaPath};

macro_rules! forward_impl {
    ( Commit $t:ty ) => {
        #[async_trait]
        impl<'a, S> Commit<PathLayer<S>> for $t
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
    };

    ( Load $t:ty ) => {
        #[async_trait]
        impl<S> Load<PathLayer<S>> for $t
        where
            S: Store,
        {
            async fn load_from_store(
                store: &PathLayer<S>,
                path: &StorePath<S::CID>,
            ) -> anyhow::Result<Self> {
                let ViaPath(x) = store.load(path).await?;
                Ok(x)
            }
        }
    };
}

forward_impl!(Commit pangalactic_hostdir::HostDirectory<S::CID>);
forward_impl!(Load pangalactic_hostdir::HostDirectory<S::CID>);
forward_impl!(Load pangalactic_hostdir::DirNodeReader<S>);
forward_impl!(Commit std::path::PathBuf);
forward_impl!(Commit &'a std::path::Path);
forward_impl!(Commit tokio::fs::File);
forward_impl!(Commit tokio::fs::ReadDir);
