use crate::{FromDag, LinkFor, ToDag, WriterFor};
use pangalactic_layer_cidmeta::CidMetaLayer;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind::File;
use pangalactic_store::Store;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Default)]
pub struct Dagio<S>(pub(crate) CidMetaLayer<S>)
where
    S: Store;

impl<S> From<S> for Dagio<S>
where
    S: Store,
{
    fn from(store: S) -> Self {
        Dagio(CidMetaLayer::from(store))
    }
}

impl<S> Dagio<S>
where
    S: Store,
{
    pub async fn read<T>(&mut self, link: &LinkFor<S>) -> anyhow::Result<T>
    where
        T: FromDag<S>,
    {
        T::from_dag(self, link).await
    }

    pub async fn commit<T>(&mut self, object: T) -> anyhow::Result<LinkFor<S>>
    where
        T: ToDag<S>,
    {
        object.into_dag(self).await
    }

    pub async fn open_file_reader(
        &mut self,
        link: &LinkFor<S>,
    ) -> anyhow::Result<<S as Store>::Reader> {
        let key = link.peek_key_kind(File)?;
        self.0.open_reader(key).await
    }

    pub async fn open_file_writer(&mut self) -> anyhow::Result<WriterFor<S>> {
        self.0.open_writer().await
    }

    pub async fn commit_file_writer(&mut self, w: WriterFor<S>) -> anyhow::Result<LinkFor<S>> {
        self.0.commit_writer(w).await.map(|k| Link::new(File, k))
    }

    pub async fn read_file_into_writer<W>(&mut self, link: &LinkFor<S>, w: W) -> anyhow::Result<u64>
    where
        W: AsyncWrite,
    {
        let mut pinw = std::pin::pin!(w);
        let mut r = self.open_file_reader(link).await?;
        let written = tokio::io::copy(&mut r, &mut pinw).await?;
        Ok(written)
    }

    pub async fn commit_file_from_reader<R>(&mut self, r: R) -> anyhow::Result<LinkFor<S>>
    where
        R: AsyncRead,
    {
        let mut pinr = std::pin::pin!(r);
        let mut w = self.open_file_writer().await?;
        tokio::io::copy(&mut pinr, &mut w).await?;
        let link = self.commit_file_writer(w).await?;
        Ok(link)
    }

    pub async fn read_file(&mut self, link: &LinkFor<S>) -> anyhow::Result<Vec<u8>> {
        let key = link.peek_key_kind(File)?;
        self.0.read(key).await
    }

    pub async fn write_file(&mut self, contents: &[u8]) -> anyhow::Result<LinkFor<S>> {
        self.0.write(contents).await.map(|k| Link::new(File, k))
    }
}
