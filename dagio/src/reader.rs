use crate::{Dagio, DagioLoad};
use async_trait::async_trait;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store::Store;
use pin_project::pin_project;
use tokio::io::AsyncRead;

#[derive(Debug)]
#[pin_project]
pub struct DagioReader<S>(#[pin] S::Reader)
where
    S: Store;

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioLoad<S> for DagioReader<S>
where
    S: Store,
{
    async fn load_from_dagio(
        dagio: &Dagio<S>,
        link: &Link<CidMeta<S::CID>>,
    ) -> anyhow::Result<Self> {
        use pangalactic_linkkind::LinkKind::File;

        let cid = link.peek_cid_kind(File)?;
        dagio.0.open_reader(cid).await.map(DagioReader)
    }
}

impl<S> AsyncRead for DagioReader<S>
where
    S: Store,
{
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.project().0.poll_read(cx, buf)
    }
}
