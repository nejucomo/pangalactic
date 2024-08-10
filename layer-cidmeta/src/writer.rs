use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;
use pangalactic_store::{Commit, Store};
use pin_project::pin_project;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::io::AsyncWrite;

use crate::{CidMeta, CidMetaLayer};

#[derive(Debug)]
#[pin_project]
pub struct Writer<W>
where
    W: AsyncWrite,
{
    #[pin]
    pub(crate) writer: W,
    pub(crate) written: usize,
}

impl<W> From<W> for Writer<W>
where
    W: AsyncWrite,
{
    fn from(writer: W) -> Self {
        Writer { writer, written: 0 }
    }
}

#[async_trait]
impl<S> Commit<CidMetaLayer<S>> for Writer<S::Writer>
where
    S: Store,
    S::CID: Serialize + DeserializeOwned,
{
    async fn commit_into_store(
        self,
        store: &mut CidMetaLayer<S>,
    ) -> anyhow::Result<CidMeta<S::CID>> {
        let Writer { writer, written } = self;

        let cid = store.0.commit(writer).await?;
        let node_size = u64::try_from(written).expect("usize->u64 conversion failure");
        Ok(CidMeta { cid, node_size })
    }
}

impl<W> AsyncWrite for Writer<W>
where
    W: AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let proj = self.project();
        let poll = proj.writer.poll_write(cx, buf);
        if let Poll::Ready(Ok(written)) = &poll {
            *proj.written += *written;
        }
        poll
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        self.project().writer.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        self.project().writer.poll_shutdown(cx)
    }
}
