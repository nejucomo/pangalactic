use std::io::IoSlice;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::AsyncWrite;

#[derive(derive_more::From)]
pub struct FileWriter<W>(W)
where
    W: Send + AsyncWrite + std::marker::Unpin;

impl<W> FileWriter<W>
where
    W: Send + AsyncWrite + std::marker::Unpin,
{
    pub(crate) fn unwrap(self) -> W {
        self.0
    }
}

impl<W> AsyncWrite for FileWriter<W>
where
    W: Send + AsyncWrite + std::marker::Unpin + std::ops::Deref,
    <W as std::ops::Deref>::Target: std::marker::Unpin,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let innerself = Pin::into_inner(self);
        let subpin = Pin::new(&mut innerself.0);

        AsyncWrite::poll_write(subpin, cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        let innerself = Pin::into_inner(self);
        let subpin = Pin::new(&mut innerself.0);

        AsyncWrite::poll_flush(subpin, cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        let innerself = Pin::into_inner(self);
        let subpin = Pin::new(&mut innerself.0);

        AsyncWrite::poll_shutdown(subpin, cx)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<std::io::Result<usize>> {
        let innerself = Pin::into_inner(self);
        let subpin = Pin::new(&mut innerself.0);

        AsyncWrite::poll_write_vectored(subpin, cx, bufs)
    }
}
