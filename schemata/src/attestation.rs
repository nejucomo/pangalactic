use async_trait::async_trait;
use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::{Dagio, FromDag, LinkFor, ToDag};
use dagwasm_dir::Directory;
use std::marker::Unpin;
use std::ops::Deref;

#[derive(Debug)]
pub struct Attestation<B>
where
    B: BlobStore,
{
    pub derivation: LinkFor<B>,
    pub output: LinkFor<B>,
}

#[async_trait]
impl<B> FromDag<B> for Attestation<B>
where
    B: BlobStore,
{
    async fn from_dag(dagio: &mut Dagio<B>, link: &LinkFor<B>) -> anyhow::Result<Self> {
        let mut dir = Directory::from_dag(dagio, link).await?;
        let derivation = dir.remove_required("derivation")?;
        let output = dir.remove_required("output")?;
        dir.require_empty()?;
        Ok(Attestation { derivation, output })
    }
}

#[async_trait]
impl<B> ToDag<B> for Attestation<B>
where
    B: BlobStore,
    <B as BlobStore>::Writer: Deref,
    <<B as BlobStore>::Writer as Deref>::Target: Unpin,
    LinkFor<B>: Clone,
{
    async fn into_dag(self, dagio: &mut Dagio<B>) -> anyhow::Result<LinkFor<B>> {
        dagio
            .commit([("derivation", self.derivation), ("output", self.output)])
            .await
    }
}
