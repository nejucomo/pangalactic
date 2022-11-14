use crate::{FileWriter, Link, LinkKind};
use dagwasm_blobstore::BlobStore;

#[derive(Debug, derive_more::From)]
pub struct Dagio<B>(B);

impl<B> Dagio<B>
where
    B: BlobStore,
    <B as BlobStore>::Writer: Send + std::marker::Unpin,
{
    pub async fn open_file_reader(
        &mut self,
        link: Link<<B as BlobStore>::Key>,
    ) -> anyhow::Result<<B as BlobStore>::Reader> {
        let key = link.unwrap_key(LinkKind::File)?;
        self.0.open_reader(key).await
    }

    pub async fn open_file_writer(
        &mut self,
    ) -> anyhow::Result<FileWriter<<B as BlobStore>::Writer>> {
        let inner = self.0.open_writer().await?;
        Ok(FileWriter::from(inner))
    }
}
