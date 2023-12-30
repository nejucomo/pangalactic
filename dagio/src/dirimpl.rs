use crate::{Dagio, DagioCommit, DagioLoad, HostDirectoryFor, LinkFor};
use async_trait::async_trait;
use pangalactic_hostdir::HostDirectory;
use pangalactic_store::Store;

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioCommit<S> for HostDirectoryFor<S>
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        use pangalactic_link::Link;
        use pangalactic_linkkind::LinkKind::Dir;
        use pangalactic_serialization::serialize;
        use tokio::io::AsyncWriteExt;

        let mut w = dagio.0.open_writer().await?;
        let buf = serialize(&self)?;
        w.write_all(&buf).await?;
        dagio
            .0
            .commit_writer(w)
            .await
            .map(|key| Link::new(Dir, key))
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<const K: usize, S, N> DagioCommit<S> for [(N, LinkFor<S>); K]
where
    S: Store,
    N: Send,
    String: From<N>,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        HostDirectory::from_iter(self.into_iter())
            .into_dag(dagio)
            .await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioLoad<S> for HostDirectoryFor<S>
where
    S: Store,
{
    async fn load_from_dagio(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        use pangalactic_link::Link;
        use pangalactic_linkkind::LinkKind::{Dir, File};
        use tokio::io::AsyncReadExt;

        let key = link.peek_key_kind(Dir)?;
        let mut r = dagio
            .open_file_reader(&Link::new(File, key.clone()))
            .await?;
        let mut buf = vec![];
        r.read_to_end(&mut buf).await?;
        let dir = pangalactic_serialization::deserialize(&buf)?;
        Ok(dir)
    }
}
