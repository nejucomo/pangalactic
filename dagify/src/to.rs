use async_trait::async_trait;
use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::{Dagio, LinkFor};
use dagwasm_dir::Directory;

#[async_trait]
pub trait ToDag<B>
where
    B: BlobStore,
{
    async fn to_dag(&self, dagio: &mut Dagio<B>) -> anyhow::Result<LinkFor<B>>;
}

#[async_trait]
impl<B> ToDag<B> for LinkFor<B>
where
    B: BlobStore,
    LinkFor<B>: Clone,
{
    async fn to_dag(&self, _: &mut Dagio<B>) -> anyhow::Result<LinkFor<B>> {
        Ok(self.clone())
    }
}

#[async_trait]
impl<B> ToDag<B> for Directory<<B as BlobStore>::Key>
where
    B: BlobStore,
    LinkFor<B>: Clone,
{
    async fn to_dag(&self, dagio: &mut Dagio<B>) -> anyhow::Result<LinkFor<B>> {
        dagio.commit_directory(self).await
    }
}
