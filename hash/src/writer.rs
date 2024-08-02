use std::{
    pin::Pin,
    task::{Context, Poll},
};

use pin_project::pin_project;
use tokio::io::AsyncWrite;

use crate::{Hash, Hasher};

#[derive(Debug)]
#[pin_project]
pub struct HashWriter<W> {
    #[pin]
    inner: W,
    hasher: Hasher,
}

impl<W> HashWriter<W> {
    pub fn unwrap(self) -> (W, Hash) {
        let HashWriter { inner, hasher } = self;
        let hash = hasher.finalize();
        (inner, hash)
    }
}

impl<W> From<W> for HashWriter<W> {
    fn from(inner: W) -> Self {
        HashWriter {
            inner,
            hasher: Hasher::default(),
        }
    }
}

impl<W> AsyncWrite for HashWriter<W>
where
    W: AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        use std::io::Write;

        let proj = self.project();
        match proj.inner.poll_write(cx, buf) {
            Poll::Ready(Ok(cnt)) => {
                proj.hasher.write_all(&buf[..cnt])?;
                Poll::Ready(Ok(cnt))
            }

            other => other,
        }
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
