use crate::{Dagio, DagioLink, DagioReader};
use async_trait::async_trait;
use pangalactic_store::Store;

#[cfg_attr(not(doc), async_trait)]
pub trait DagioLoad<S>: Sized
where
    S: Store,
{
    async fn load_from_dagio(dagio: &Dagio<S>, link: &DagioLink<S>) -> anyhow::Result<Self>;
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioLoad<S> for DagioLink<S>
where
    S: Store,
    DagioLink<S>: Clone,
{
    async fn load_from_dagio(_: &Dagio<S>, link: &DagioLink<S>) -> anyhow::Result<Self> {
        Ok(link.clone())
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioLoad<S> for Vec<u8>
where
    S: Store,
{
    async fn load_from_dagio(dagio: &Dagio<S>, link: &DagioLink<S>) -> anyhow::Result<Self> {
        use tokio::io::AsyncReadExt;

        let mut r: DagioReader<S> = dagio.load(link).await?;
        let mut buf = vec![];
        r.read_to_end(&mut buf).await?;
        Ok(buf)
    }
}
