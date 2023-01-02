use dagwasm_dir::Name;
use std::io::Cursor;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, ReadBuf};

#[derive(Debug)]
pub(crate) struct ByteReader(Cursor<Vec<u8>>);

impl From<Name> for ByteReader {
    fn from(name: Name) -> Self {
        ByteReader(Cursor::new(name.into_bytes()))
    }
}

impl AsyncRead for ByteReader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let mutself = Pin::into_inner(self);
        let pincursor = Pin::new(&mut mutself.0);
        AsyncRead::poll_read(pincursor, cx, buf)
    }
}
