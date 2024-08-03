use async_trait::async_trait;
use pangalactic_store::{Load, Store};
use pin_project::pin_project;
use serde::{de::DeserializeOwned, Serialize};
use tokio::io::AsyncRead;

use crate::{CidMeta, CidMetaLayer};

#[pin_project]
#[derive(Debug)]
pub struct Reader<R>(#[pin] R);

#[async_trait]
impl<S> Load<CidMetaLayer<S>> for Reader<S::Reader>
where
    S: Store,
    S::CID: Serialize + DeserializeOwned,
{
    async fn load_from_store(
        store: &CidMetaLayer<S>,
        cid: &CidMeta<S::CID>,
    ) -> anyhow::Result<Self> {
        let inner: S::Reader = store.0.load(&cid.cid).await?;
        Ok(Reader(inner))
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
