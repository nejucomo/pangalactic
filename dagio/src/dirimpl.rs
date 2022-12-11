use crate::{Dagio, LinkFor};
use crate::{FromDag, ToDag};
use async_trait::async_trait;
use dagwasm_blobstore::BlobStore;
use dagwasm_dir::Directory;
use std::marker::Unpin;
use std::ops::Deref;

#[async_trait]
impl<B> ToDag<B> for Directory<<B as BlobStore>::Key>
where
    B: BlobStore,
    <B as BlobStore>::Writer: Deref,
    <<B as BlobStore>::Writer as Deref>::Target: Unpin,
{
    async fn into_dag(self, dagio: &mut Dagio<B>) -> anyhow::Result<LinkFor<B>> {
        use dagwasm_dir::{Link, LinkKind::Dir};
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
impl<const K: usize, B, N> ToDag<B> for [(N, LinkFor<B>); K]
where
    B: BlobStore,
    <B as BlobStore>::Writer: Deref,
    <<B as BlobStore>::Writer as Deref>::Target: Unpin,
    N: Send,
    String: From<N>,
{
    async fn into_dag(self, dagio: &mut Dagio<B>) -> anyhow::Result<LinkFor<B>> {
        Directory::from_iter(self.into_iter()).into_dag(dagio).await
    }
}

#[async_trait]
impl<B> FromDag<B> for Directory<<B as BlobStore>::Key>
where
    B: BlobStore,
{
    async fn from_dag(dagio: &mut Dagio<B>, link: &LinkFor<B>) -> anyhow::Result<Self> {
        use dagwasm_dir::{
            Link,
            LinkKind::{Dir, File},
        };
        use dagwasm_serialization::AsyncDeserialize;

        let key = link.peek_key(Dir)?;
        let r = dagio
            .open_file_reader(&Link::new(File, key.clone()))
            .await?;
        let dir = Directory::read_from(r).await?;
        Ok(dir)
    }
}
