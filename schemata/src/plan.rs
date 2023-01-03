use async_trait::async_trait;
use dagwasm_dagio::{Dagio, FromDag, LinkFor, ToDag};
use dagwasm_dir::Directory;
use dagwasm_store::Store;
use std::marker::Unpin;
use std::ops::Deref;

#[derive(Debug)]
pub struct Plan<S>
where
    S: Store,
{
    pub exec: LinkFor<S>,
    pub input: LinkFor<S>,
}

#[async_trait]
impl<S> FromDag<S> for Plan<S>
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
impl<S> ToDag<S> for Plan<S>
where
    S: Store,
    <S as Store>::Writer: Deref,
    <<S as Store>::Writer as Deref>::Target: Unpin,
    LinkFor<S>: Clone,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        dagio
            .commit([("exec", self.exec), ("input", self.input)])
            .await
    }
}
