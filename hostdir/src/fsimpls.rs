//! Implementations of [Commit] for `std::path` types

use std::path::{Path, PathBuf};

use anyhow_std::OsStrAnyhow;
use pangalactic_iowrappers::Readable;
use pangalactic_link::Link;
use pangalactic_store::{Commit, Store};
use tokio::fs::{self, File, ReadDir};

use crate::{HostDirectory, HostDirectoryLayer};

impl<S> Commit<HostDirectoryLayer<S>> for PathBuf
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut HostDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        store.commit(self.as_path()).await
    }
}

impl<'a, S> Commit<HostDirectoryLayer<S>> for &'a Path
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut HostDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        if self.is_file() {
            let f = File::open(self).await?;
            store.commit(f).await
        } else if self.is_dir() {
            let r = fs::read_dir(&self).await?;
            Box::pin(store.commit(r)).await
        } else {
            anyhow::bail!("unsupported fs node type: {:?}", self.display())
        }
    }
}

impl<S> Commit<HostDirectoryLayer<S>> for ReadDir
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut HostDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        let mut reader = self;
        let mut d = HostDirectory::default();
        while let Some(entry) = reader.next_entry().await? {
            let name = entry.file_name().to_str_anyhow()?.to_string();
            let link = store.commit(entry.path()).await?;
            d.insert(name, link)?;
        }
        store.commit(d).await
    }
}

impl<S> Commit<HostDirectoryLayer<S>> for File
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut HostDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        store.commit(Readable(self)).await
    }
}
