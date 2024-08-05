use async_trait::async_trait;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind;
use pangalactic_store::{Load, Store};
use pin_project::pin_project;
use tokio::io::AsyncRead;

use crate::HostDirectoryLayer;

#[pin_project]
#[derive(Debug)]
pub struct Reader<R>(#[pin] R);

impl<R> Reader<R> {
    pub(crate) fn new(r: R) -> Self {
        Reader(r)
    }

    pub fn unwrap(self) -> R {
        self.0
    }
}

#[async_trait]
impl<S> Load<HostDirectoryLayer<S>> for Reader<S::Reader>
where
    S: Store,
{
    async fn load_from_store(
        store: &HostDirectoryLayer<S>,
        link: &Link<S::CID>,
    ) -> anyhow::Result<Self> {
        store.open_kind_reader(link, LinkKind::File).await
    }
}

impl<R> AsyncRead for Reader<R>
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
