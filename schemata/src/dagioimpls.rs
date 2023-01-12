use crate::{Attestation, Plan};
use async_trait::async_trait;
use dagwasm_dagio::{Dagio, FromDag, LinkFor, ToDag};
use dagwasm_dir::Directory;
use dagwasm_store::Store;

#[async_trait]
impl<S> FromDag<S> for Attestation<LinkFor<S>>
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
impl<S> ToDag<S> for Attestation<LinkFor<S>>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        dagio
            .commit([("plan", self.plan), ("output", self.output)])
            .await
    }
}

#[async_trait]
impl<S> FromDag<S> for Plan<LinkFor<S>>
where
    S: Store,
{
    async fn from_dag(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        let mut dir = Directory::from_dag(dagio, link).await?;
        let exec = dir.remove_required("exec")?;
        let input = dir.remove_required("input")?;
        dir.require_empty()?;
        Ok(Plan { exec, input })
    }
}

#[async_trait]
impl<S> ToDag<S> for Plan<LinkFor<S>>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        dagio
            .commit([("exec", self.exec), ("input", self.input)])
            .await
    }
}
