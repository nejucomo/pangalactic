use async_trait::async_trait;
use dagwasm_dagio::{Dagio, FromDag, LinkFor, ToDag};
use dagwasm_dir::Directory;
use dagwasm_store::Store;
use std::marker::Unpin;
use std::ops::Deref;

#[derive(Debug)]
pub struct Attestation<S>
where
    S: Store,
{
    pub plan: LinkFor<S>,
    pub output: LinkFor<S>,
}

#[async_trait]
impl<S> FromDag<S> for Attestation<S>
where
    S: Store,
{
    async fn from_dag(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        let mut dir = Directory::from_dag(dagio, link).await?;
        let plan = dir.remove_required("plan")?;
        let output = dir.remove_required("output")?;
        dir.require_empty()?;
        Ok(Attestation { plan, output })
    }
}

#[async_trait]
impl<S> ToDag<S> for Attestation<S>
where
    S: Store,
    <S as Store>::Writer: Deref,
    <<S as Store>::Writer as Deref>::Target: Unpin,
    LinkFor<S>: Clone,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        dagio
            .commit([("plan", self.plan), ("output", self.output)])
            .await
    }
}
