use async_trait::async_trait;
use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::{Dagio, FromDag, LinkFor};
use dagwasm_dir::Directory;

#[derive(Debug)]
pub(crate) struct DirectoryReader<B>(<Directory<<B as BlobStore>::Key> as IntoIterator>::IntoIter)
where
    B: BlobStore;

#[async_trait]
impl<B> FromDag<B> for DirectoryReader<B>
where
    B: BlobStore,
{
    async fn from_dag(dagio: &mut Dagio<B>, link: &LinkFor<B>) -> anyhow::Result<Self> {
        let dir: Directory<<B as BlobStore>::Key> = dagio.read(link).await?;
        Ok(DirectoryReader(dir.into_iter()))
    }
}
