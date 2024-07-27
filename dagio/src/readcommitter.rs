use async_trait::async_trait;
use pangalactic_store::Store;
use tokio::io::AsyncRead;

use crate::{Dagio, DagioCommit, DagioLink};

#[derive(Debug)]
pub struct DagioReadCommitter<R>(pub R);

#[cfg_attr(not(doc), async_trait)]
impl<R, S> DagioCommit<S> for DagioReadCommitter<R>
where
    S: Store,
    R: AsyncRead + Send,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        let mut w = dagio.open_file_writer().await?;
        tokio::io::copy(&mut std::pin::pin!(self.0), &mut w).await?;
        dagio.commit(w).await
    }
}
