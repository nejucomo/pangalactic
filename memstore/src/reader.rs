use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, ReadBuf};

#[derive(Debug)]
pub struct Reader(Arc<Vec<u8>>);

impl Reader {
    pub(crate) fn new(contents: Vec<u8>) -> Self {
        Reader(Arc::new(contents))
    }
}

impl Clone for Reader {
    fn clone(&self) -> Self {
        Reader(self.0.clone())
    }
}

impl AsyncRead for Reader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let mutself = Pin::into_inner(self);
        let mut bytes = mutself.0.as_slice();
        let pinbytes = Pin::new(&mut bytes);
        AsyncRead::poll_read(pinbytes, cx, buf)
    }
}
