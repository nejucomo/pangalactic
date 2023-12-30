use crate::{Dagio, DagioLink};
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
        use tokio::io::AsyncWriteExt;

        let mut w = dagio.open_file_writer().await?;
        w.write_all(self).await?;
        dagio.commit(w).await
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
