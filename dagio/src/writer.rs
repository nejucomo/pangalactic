use crate::{Dagio, DagioCommit, DagioLink};
use async_trait::async_trait;
use pangalactic_layer_cidmeta::CidMetaLayer;
use pangalactic_store::Store;
use pin_project::pin_project;
use tokio::io::AsyncWrite;

#[pin_project]
pub struct DagioWriter<S>(#[pin] <CidMetaLayer<S> as Store>::Writer)
where
    S: Store;

impl<S> DagioWriter<S>
where
    S: Store,
{
    pub(crate) fn new(inner: <CidMetaLayer<S> as Store>::Writer) -> Self {
        DagioWriter(inner)
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioCommit<S> for DagioWriter<S>
where
    S: Store,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        use pangalactic_link::Link;
        use pangalactic_linkkind::LinkKind::File;

        dagio
            .0
            .commit_writer(self.0)
            .await
            .map(|k| Link::new(File, k))
    }
}

impl<S> AsyncWrite for DagioWriter<S>
where
    S: Store,
{
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        self.project().0.poll_write(cx, buf)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        self.project().0.poll_flush(cx)
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        self.project().0.poll_shutdown(cx)
    }
}
