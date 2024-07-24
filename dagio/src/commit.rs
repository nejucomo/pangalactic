use std::path::Path;

use crate::{Dagio, DagioLink, DagioReadCommitter};
use async_trait::async_trait;
use pangalactic_store::Store;

#[cfg_attr(not(doc), async_trait)]
pub trait DagioCommit<S>
where
    S: Store,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>>;
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioCommit<S> for DagioLink<S>
where
    S: Store,
    DagioLink<S>: Clone,
{
    async fn commit_into_dagio(self, _: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        Ok(self)
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<'a, S> DagioCommit<S> for &'a [u8]
where
    S: Store,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        dagio.commit(DagioReadCommitter(self)).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioCommit<S> for Vec<u8>
where
    S: Store,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        dagio.commit(self.as_slice()).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<'a, S> DagioCommit<S> for &'a Path
where
    S: Store,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        if self.is_file() {
            let f = tokio::fs::File::open(self).await?;
            dagio.commit(f).await
        } else if self.is_dir() {
            let rd = tokio::fs::read_dir(self).await?;
            dagio.commit(rd).await
        } else {
            anyhow::bail!("Unknown host fs node type: {:?}", self.display())
        }
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioCommit<S> for tokio::fs::File
where
    S: Store,
{
    async fn commit_into_dagio(mut self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        dagio.commit(DagioReadCommitter(self)).await
    }
}
