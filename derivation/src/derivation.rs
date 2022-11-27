use async_trait::async_trait;
use dagwasm_blobstore::BlobStore;
use dagwasm_dagify::FromDag;
use dagwasm_dagio::{Dagio, LinkFor};

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
        let mut dir = dagio.read_directory(link).await?;
        let exec = dir.remove_required("exec")?;
        let input = dir.remove_required("exec")?;
        dir.require_empty()?;
        Ok(Derivation { exec, input })
    }
}
