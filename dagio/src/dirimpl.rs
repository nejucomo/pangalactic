use crate::{Dagio, DirectoryFor, FromDag, LinkFor, ToDag};
use async_trait::async_trait;
use dagwasm_dir::Directory;
use dagwasm_store::Store;

#[async_trait]
impl<S> ToDag<S> for DirectoryFor<S>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        use dagwasm_link::Link;
        use dagwasm_linkkind::LinkKind::Dir;
        use dagwasm_serialization::AsyncSerialize;

        let mut w = dagio.0.open_writer().await?;
        self.write_into(&mut w).await?;
        dagio
            .0
            .commit_writer(w)
            .await
            .map(|key| Link::new(Dir, key))
    }
}

#[async_trait]
impl<const K: usize, S, N> ToDag<S> for [(N, LinkFor<S>); K]
where
    S: Store,
    N: Send,
    String: From<N>,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        Directory::from_iter(self.into_iter()).into_dag(dagio).await
    }
}

#[async_trait]
impl<S> FromDag<S> for DirectoryFor<S>
where
    S: Store,
{
    async fn from_dag(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        use dagwasm_link::Link;
        use dagwasm_linkkind::LinkKind::{Dir, File};
        use dagwasm_serialization::AsyncDeserialize;

        let key = link.peek_key_kind(Dir)?;
        let r = dagio
            .open_file_reader(&Link::new(File, key.clone()))
            .await?;
        let dir = Directory::read_from(r).await?;
        Ok(dir)
    }
}
