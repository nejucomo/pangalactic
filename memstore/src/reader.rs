use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, ReadBuf};

#[derive(Debug)]
pub struct Reader {
    data: Arc<Vec<u8>>,
    readcnt: usize,
}

impl Reader {
    pub(crate) fn new(contents: Arc<Vec<u8>>) -> Self {
        Reader {
            data: contents,
            readcnt: 0,
        }
    }
}

impl AsyncRead for Reader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        // async plumbing:
        let bufprelen = buf.filled().len();
        let mutself = Pin::into_inner(self);

        // subslice our data based on our readcnt:
        let mut bytes = &mutself.data.as_slice()[mutself.readcnt..];
        let pinbytes = Pin::new(&mut bytes);

        let res = AsyncRead::poll_read(pinbytes, cx, buf);

        // Update our read count if downstream was ready:
        if let Poll::Ready(Ok(())) = &res {
            let fillcnt = buf.filled().len() - bufprelen;
            mutself.readcnt += fillcnt;
        }

        res
    }
}
