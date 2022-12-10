use async_trait::async_trait;
use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::{Dagio, FromDag, LinkFor, ToDag};
use dagwasm_dir::Directory;
use std::marker::Unpin;
use std::ops::Deref;

#[derive(Debug)]
pub struct Derivation<B>
where
    B: BlobStore,
{
    pub exec: LinkFor<B>,
    pub input: LinkFor<B>,
}

#[async_trait]
impl<B> FromDag<B> for Derivation<B>
where
    B: BlobStore,
{
    async fn from_dag(dagio: &mut Dagio<B>, link: &LinkFor<B>) -> anyhow::Result<Self> {
        let mut dir = Directory::from_dag(dagio, link).await?;
        let exec = dir.remove_required("exec")?;
        let input = dir.remove_required("input")?;
        dir.require_empty()?;
        Ok(Derivation { exec, input })
    }
}

#[async_trait]
impl<B> ToDag<B> for Derivation<B>
where
    B: BlobStore,
    <B as BlobStore>::Writer: Deref,
    <<B as BlobStore>::Writer as Deref>::Target: Unpin,
    LinkFor<B>: Clone,
{
    async fn to_dag(&self, dagio: &mut Dagio<B>) -> anyhow::Result<LinkFor<B>> {
        Directory::from_iter([("exec", self.exec.clone()), ("input", self.input.clone())])
            .to_dag(dagio)
            .await
    }
}
