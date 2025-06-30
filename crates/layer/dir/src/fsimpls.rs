//! Implementations of [Commit] for `std::path` types

use std::path::{Path, PathBuf};

use anyhow_std::OsStrAnyhow;
use pangalactic_iowrappers::Readable;
use pangalactic_link::Link;
use pangalactic_store::{Commit, Store};
use tokio::fs::{self, File, ReadDir};

use crate::{LinkDirectory, LinkDirectoryLayer};

impl<S> Commit<LinkDirectoryLayer<S>> for PathBuf
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        let link = store.commit(self.as_path()).await?;
        tracing::debug!(path=?&self, ?link, "committed");
        Ok(link)
    }
}

impl<S> Commit<LinkDirectoryLayer<S>> for &Path
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
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

impl<S> Commit<LinkDirectoryLayer<S>> for ReadDir
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        let mut reader = self;
        let mut d = LinkDirectory::default();
        while let Some(entry) = reader.next_entry().await? {
            let name = entry.file_name().to_str_anyhow()?.to_string();
            let link = store.commit(entry.path()).await?;
            d.insert(name, link)?;
        }
        store.commit(d).await
    }
}

impl<S> Commit<LinkDirectoryLayer<S>> for File
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        store.commit(Readable(self)).await
    }
}
