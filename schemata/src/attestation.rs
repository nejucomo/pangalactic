use async_trait::async_trait;
use dagwasm_dagio::{Dagio, FromDag, LinkFor, ToDag};
use dagwasm_dir::Directory;
use dagwasm_store::Store;
use std::marker::Unpin;
use std::ops::Deref;

#[derive(Debug)]
pub struct Attestation<B>
where
    B: Store,
{
    pub plan: LinkFor<B>,
    pub output: LinkFor<B>,
}

#[async_trait]
impl<B> FromDag<B> for Attestation<B>
where
    B: Store,
{
    async fn from_dag(dagio: &mut Dagio<B>, link: &LinkFor<B>) -> anyhow::Result<Self> {
        let mut dir = Directory::from_dag(dagio, link).await?;
        let plan = dir.remove_required("plan")?;
        let output = dir.remove_required("output")?;
        dir.require_empty()?;
        Ok(Attestation { plan, output })
    }
}

#[async_trait]
impl<B> ToDag<B> for Attestation<B>
where
    B: Store,
    <B as Store>::Writer: Deref,
    <<B as Store>::Writer as Deref>::Target: Unpin,
    LinkFor<B>: Clone,
{
    async fn into_dag(self, dagio: &mut Dagio<B>) -> anyhow::Result<LinkFor<B>> {
        dagio
            .commit([("plan", self.plan), ("output", self.output)])
            .await
    }
}
