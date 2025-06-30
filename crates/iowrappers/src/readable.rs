use pin_project::pin_project;
use tokio::io::AsyncRead;

#[pin_project]
#[derive(Debug)]
pub struct Readable<R>(#[pin] pub R);

impl<R> AsyncRead for Readable<R>
where
    R: AsyncRead,
{
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.project().0.poll_read(cx, buf)
    }
}
