use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::AsyncWrite;

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
