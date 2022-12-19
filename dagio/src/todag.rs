use crate::{Dagio, LinkFor};
use async_trait::async_trait;
use dagwasm_blobstore::BlobStore;

#[async_trait]
pub trait ToDag<B>
where
    B: BlobStore,
{
    async fn into_dag(self, dagio: &mut Dagio<B>) -> anyhow::Result<LinkFor<B>>;
}

#[async_trait]
impl<B> ToDag<B> for LinkFor<B>
where
    B: BlobStore,
    LinkFor<B>: Clone,
{
    async fn into_dag(self, _: &mut Dagio<B>) -> anyhow::Result<LinkFor<B>> {
        Ok(self)
    }
}
