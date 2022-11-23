use crate::FileWriter;
use dagwasm_blobstore::BlobStore;
use dagwasm_dir::{Link, LinkKind::File};

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
        let key = link.unwrap_key(File)?;
        self.0.open_reader(key).await
    }

    pub async fn open_file_writer(
        &mut self,
    ) -> anyhow::Result<FileWriter<<B as BlobStore>::Writer>> {
        let inner = self.0.open_writer().await?;
        Ok(FileWriter::from(inner))
    }

    pub async fn commit_file_writer(
        &mut self,
        w: FileWriter<<B as BlobStore>::Writer>,
    ) -> anyhow::Result<Link<<B as BlobStore>::Key>> {
        self.0
            .commit_writer(w.unwrap())
            .await
            .map(|k| Link::new(File, k))
    }
}
