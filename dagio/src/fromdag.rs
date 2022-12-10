use crate::{Dagio, LinkFor};
use async_trait::async_trait;
use dagwasm_blobstore::BlobStore;
use dagwasm_dir::Directory;

#[async_trait]
pub trait FromDag<B>: Sized
where
    B: BlobStore,
{
    async fn from_dag(dagio: &mut Dagio<B>, link: &LinkFor<B>) -> anyhow::Result<Self>;
}

#[async_trait]
impl<B> FromDag<B> for LinkFor<B>
where
    B: BlobStore,
    LinkFor<B>: Clone,
{
    async fn from_dag(_: &mut Dagio<B>, link: &LinkFor<B>) -> anyhow::Result<Self> {
        Ok(link.clone())
    }
}

#[async_trait]
impl<B> FromDag<B> for Directory<<B as BlobStore>::Key>
where
    B: BlobStore,
{
    async fn from_dag(dagio: &mut Dagio<B>, link: &LinkFor<B>) -> anyhow::Result<Self> {
        dagio.read_directory(link).await
    }
}
