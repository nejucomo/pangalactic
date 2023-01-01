use async_trait::async_trait;
use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::{Dagio, FromDag, LinkFor};
use dagwasm_dir::{Directory, Name};

#[derive(Debug)]
pub(crate) struct DirectoryReader<B>
where
    B: BlobStore,
{
    #[allow(dead_code)]
    iter: <Directory<<B as BlobStore>::Key> as IntoIterator>::IntoIter,
    next: Option<(Name, LinkFor<B>)>,
}

impl<B> DirectoryReader<B>
where
    B: BlobStore,
{
    pub(crate) fn has_more_entries(&self) -> bool {
        self.next.is_some()
    }
}

#[async_trait]
impl<B> FromDag<B> for DirectoryReader<B>
where
    B: BlobStore,
{
    async fn from_dag(dagio: &mut Dagio<B>, link: &LinkFor<B>) -> anyhow::Result<Self> {
        let dir: Directory<<B as BlobStore>::Key> = dagio.read(link).await?;
        let mut iter = dir.into_iter();
        let next = iter.next();
        Ok(DirectoryReader { iter, next })
    }
}
