use crate::{Dagio, DagioCommit, DagioHostDirectory, DagioLink, ReadCommitter};
use async_trait::async_trait;
use pangalactic_store::Store;
use std::path::{Path, PathBuf};

#[cfg_attr(not(doc), async_trait)]
impl<'a, S> DagioCommit<S> for &'a Path
where
    S: Store,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        use anyhow_std::OsStrAnyhow;

        if self.is_file() {
            let f = tokio::fs::File::open(self).await?;
            dagio.commit(f).await
        } else if self.is_dir() {
            let mut hd = DagioHostDirectory::default();
            let mut rdir = tokio::fs::read_dir(self).await?;
            while let Some(entry) = rdir.next_entry().await? {
                let nameos = entry.file_name();
                let name = nameos.to_str_anyhow()?;
                let link = dagio.commit(entry.path()).await?;
                hd.insert(name.to_string(), link)?;
            }
            dagio.commit(hd).await
        } else {
            anyhow::bail!("cannot commit non-file/non-dir {:?}", self.display());
        }
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioCommit<S> for PathBuf
where
    S: Store,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        dagio.commit(self.as_path()).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioCommit<S> for tokio::fs::File
where
    S: Store,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        dagio.commit(ReadCommitter(self)).await
    }
}
