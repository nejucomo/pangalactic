use async_trait::async_trait;
use dagwasm_dagio::{Dagio, FromDag, LinkFor, ToDag};
use dagwasm_dir::Directory;
use dagwasm_store::Store;
use std::marker::Unpin;
use std::ops::Deref;

#[derive(Debug)]
pub struct Plan<B>
where
    B: Store,
{
    pub exec: LinkFor<B>,
    pub input: LinkFor<B>,
}

#[async_trait]
impl<B> FromDag<B> for Plan<B>
where
    B: Store,
{
    async fn from_dag(dagio: &mut Dagio<B>, link: &LinkFor<B>) -> anyhow::Result<Self> {
        let mut dir = Directory::from_dag(dagio, link).await?;
        let exec = dir.remove_required("exec")?;
        let input = dir.remove_required("input")?;
        dir.require_empty()?;
        Ok(Plan { exec, input })
    }
}

#[async_trait]
impl<B> ToDag<B> for Plan<B>
where
    B: Store,
    <B as Store>::Writer: Deref,
    <<B as Store>::Writer as Deref>::Target: Unpin,
    LinkFor<B>: Clone,
{
    async fn into_dag(self, dagio: &mut Dagio<B>) -> anyhow::Result<LinkFor<B>> {
        dagio
            .commit([("exec", self.exec), ("input", self.input)])
            .await
    }
}
