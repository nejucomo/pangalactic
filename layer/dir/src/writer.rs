use std::pin::Pin;
use std::task::{Context, Poll};

use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind;
use pangalactic_store::{Commit, Store};
use pin_project::pin_project;
use tokio::io::AsyncWrite;

use crate::LinkDirectoryLayer;

#[pin_project]
#[derive(Debug, derive_more::From)]
pub struct Writer<T> {
    kind: LinkKind,
    #[pin]
    inner: T,
}

impl<T> Writer<T> {
    pub(crate) fn new(kind: LinkKind, inner: T) -> Self {
        Writer { kind, inner }
    }
}

impl<S> Commit<LinkDirectoryLayer<S>> for Writer<S::Writer>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        store.commit_inner(self.kind, self.inner).await
    }
}

impl<T> AsyncWrite for Writer<T>
where
    T: AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        self.project().inner.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        self.project().inner.poll_shutdown(cx)
    }
}
