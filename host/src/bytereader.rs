use pangalactic_layer_storedir::Name;
use pangalactic_store::Store;
use std::io::Cursor;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, ReadBuf};

use crate::store::HostReader;

pub(crate) enum ByteReader<S>
where
    S: Store,
{
    Buf(Cursor<Vec<u8>>),
    Store(HostReader<S>),
}

impl<S> From<Name> for ByteReader<S>
where
    S: Store,
{
    fn from(name: Name) -> Self {
        ByteReader::Buf(Cursor::new(name.into_bytes()))
    }
}

impl<S> AsyncRead for ByteReader<S>
where
    S: Store,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match Pin::into_inner(self) {
            ByteReader::Buf(cursor) => {
                let pin = Pin::new(cursor);
                AsyncRead::poll_read(pin, cx, buf)
            }
            ByteReader::Store(reader) => {
                let pin = Pin::new(reader);
                AsyncRead::poll_read(pin, cx, buf)
            }
        }
    }
}
