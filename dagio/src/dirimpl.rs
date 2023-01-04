use crate::{Dagio, LinkFor};
use crate::{FromDag, ToDag};
use async_trait::async_trait;
use dagwasm_dir::Directory;
use dagwasm_store::Store;
use std::marker::Unpin;
use std::ops::Deref;

#[async_trait]
impl<S> ToDag<S> for Directory<<S as Store>::CID>
where
    S: Store,
    <S as Store>::Writer: Deref,
    <<S as Store>::Writer as Deref>::Target: Unpin,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        use dagwasm_dir::Link;
        use dagwasm_linkkind::LinkKind::Dir;
        use dagwasm_serialization::AsyncSerialize;

        let mut w = dagio.open_file_writer().await?;
        self.write_into(&mut w).await?;
        dagio
            .commit_file_writer(w)
            .await
            // Transmute the file link into a dir link:
            .map(Link::unwrap)
            .map(|(_, key)| Link::new(Dir, key))
    }
}

#[async_trait]
impl<const K: usize, S, N> ToDag<S> for [(N, LinkFor<S>); K]
where
    S: Store,
    <S as Store>::Writer: Deref,
    <<S as Store>::Writer as Deref>::Target: Unpin,
    N: Send,
    String: From<N>,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        Directory::from_iter(self.into_iter()).into_dag(dagio).await
    }
}

#[async_trait]
impl<S> FromDag<S> for Directory<<S as Store>::CID>
where
    S: Store,
{
    async fn from_dag(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        use dagwasm_dir::Link;
        use dagwasm_linkkind::LinkKind::{Dir, File};
        use dagwasm_serialization::AsyncDeserialize;

        let key = link.peek_key(Dir)?;
        let r = dagio
            .open_file_reader(&Link::new(File, key.clone()))
            .await?;
        let dir = Directory::read_from(r).await?;
        Ok(dir)
    }
}
