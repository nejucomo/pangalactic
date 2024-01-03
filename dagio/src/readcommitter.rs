use async_trait::async_trait;
use pangalactic_store::Store;
use tokio::io::AsyncRead;

use crate::{Dagio, DagioCommit, DagioLink};

/// A `ReadCommitter<R>` reads all of `R` into a new store file and commits it upon commit.
#[derive(Debug)]
pub struct ReadCommitter<R>(pub R)
where
    R: AsyncRead + Send;

#[cfg_attr(not(doc), async_trait)]
impl<S, R> DagioCommit<S> for ReadCommitter<R>
where
    S: Store,
    R: AsyncRead + Send,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        let mut pinr = std::pin::pin!(self.0);
        let mut w = dagio.open_file_writer().await?;
        tokio::io::copy(&mut pinr, &mut w).await?;
        dagio.commit(w).await
    }
}
