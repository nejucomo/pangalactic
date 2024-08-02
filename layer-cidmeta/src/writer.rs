use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;
use pangalactic_store::{Commit, Store};
use tokio::io::AsyncWrite;

use crate::{CidMeta, CidMetaLayer};

#[derive(Debug)]
pub struct Writer<W>
where
    W: AsyncWrite + Unpin,
{
    pub(crate) writer: W,
    pub(crate) written: usize,
}

impl<W> AsyncWrite for Writer<W>
where
    // BUG: Replace `Unpin` bound with `pin_project`
    W: AsyncWrite + Unpin,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let mutself = Pin::get_mut(self);
        let pininner = Pin::new(&mut mutself.writer);
        let poll = AsyncWrite::poll_write(pininner, cx, buf);
        if let Poll::Ready(Ok(written)) = &poll {
            mutself.written += *written;
        }
        poll
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        let mutself = Pin::get_mut(self);
        let pininner = Pin::new(&mut mutself.writer);
        AsyncWrite::poll_flush(pininner, cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        let mutself = Pin::get_mut(self);
        let pininner = Pin::new(&mut mutself.writer);
        AsyncWrite::poll_shutdown(pininner, cx)
    }
}

#[async_trait]
impl<S> Commit<CidMetaLayer<S>> for Writer<S::Writer>
where
    S: Store,
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
