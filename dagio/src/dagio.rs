use crate::{FileWriter, LinkFor};
use dagwasm_blobstore::BlobStore;
use dagwasm_dir::{
    Directory, Link,
    LinkKind::{Dir, File},
};

#[derive(Debug, derive_more::From)]
pub struct Dagio<B>(B);

impl<B> Dagio<B>
where
    B: BlobStore,
    <B as BlobStore>::Writer: Send + std::marker::Unpin,
{
    pub async fn open_file_reader(
        &mut self,
        link: &LinkFor<B>,
    ) -> anyhow::Result<<B as BlobStore>::Reader> {
        let key = link.peek_key(File)?;
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
    ) -> anyhow::Result<LinkFor<B>> {
        self.0
            .commit_writer(w.unwrap())
            .await
            .map(|k| Link::new(File, k))
    }

    pub async fn read_file(&mut self, link: &LinkFor<B>) -> anyhow::Result<Vec<u8>> {
        let key = link.peek_key(File)?;
        self.0.read(key).await
    }

    pub async fn read_directory(
        &mut self,
        link: &LinkFor<B>,
    ) -> anyhow::Result<Directory<<B as BlobStore>::Key>> {
        use dagwasm_serialization::AsyncDeserialize;

        let key = link.peek_key(Dir)?;
        let r = self.0.open_reader(key).await?;
        let dir = Directory::read_from(r).await?;
        Ok(dir)
    }

    pub async fn write_file(&mut self, contents: &[u8]) -> anyhow::Result<LinkFor<B>> {
        self.0.write(contents).await.map(|k| Link::new(File, k))
    }

    pub async fn commit_directory(
        &mut self,
        dir: &Directory<<B as BlobStore>::Key>,
    ) -> anyhow::Result<LinkFor<B>> {
        use dagwasm_serialization::AsyncSerialize;

        let mut w = self.0.open_writer().await?;
        dir.write_into(&mut w).await?;
        self.0.commit_writer(w).await.map(|k| Link::new(Dir, k))
    }
}
