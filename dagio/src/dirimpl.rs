use crate::{Dagio, DagioCommit, DagioHostDirectory, DagioLink, DagioLoad};
use async_trait::async_trait;
use pangalactic_hostdir::HostDirectory;
use pangalactic_store::Store;

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioCommit<S> for DagioHostDirectory<S>
where
    S: Store,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
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
impl<const K: usize, S, N> DagioCommit<S> for [(N, DagioLink<S>); K]
where
    S: Store,
    N: Send,
    String: From<N>,
{
    async fn commit_into_dagio(self, dagio: &mut Dagio<S>) -> anyhow::Result<DagioLink<S>> {
        dagio
            .commit(HostDirectory::from_iter(self.into_iter()))
            .await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioLoad<S> for DagioHostDirectory<S>
where
    S: Store,
{
    async fn load_from_dagio(dagio: &mut Dagio<S>, link: &DagioLink<S>) -> anyhow::Result<Self> {
        use pangalactic_link::Link;
        use pangalactic_linkkind::LinkKind::{Dir, File};

        let key = link.peek_key_kind(Dir)?;
        let translink = Link::new(File, key.clone());
        let bytes: Vec<u8> = dagio.load(&translink).await?;
        let dir = pangalactic_serialization::deserialize(&bytes)?;
        Ok(dir)
    }
}
