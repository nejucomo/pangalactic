use pin_project::pin_project;
use tokio::io::AsyncRead;

#[pin_project]
#[derive(Debug)]
pub struct Stdin(#[pin] tokio::io::Stdin);

impl Default for Stdin {
    fn default() -> Self {
        Stdin(tokio::io::stdin())
    }
}

impl AsyncRead for Stdin {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.project().0.poll_read(cx, buf)
    }
}
