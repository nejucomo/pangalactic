use crate::{Dagio, DagioReader};
use async_trait::async_trait;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store::Store;

#[cfg_attr(not(doc), async_trait)]
pub trait DagioLoad<S>: Sized
where
    S: Store,
{
    async fn load_from_dagio(
        dagio: &Dagio<S>,
        link: &Link<CidMeta<S::CID>>,
    ) -> anyhow::Result<Self>;
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioLoad<S> for Link<CidMeta<S::CID>>
where
    S: Store,
    Link<CidMeta<S::CID>>: Clone,
{
    async fn load_from_dagio(_: &Dagio<S>, link: &Link<CidMeta<S::CID>>) -> anyhow::Result<Self> {
        Ok(link.clone())
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioLoad<S> for Vec<u8>
where
    S: Store,
{
    async fn load_from_dagio(
        dagio: &Dagio<S>,
        link: &Link<CidMeta<S::CID>>,
    ) -> anyhow::Result<Self> {
        use tokio::io::AsyncReadExt;

        let mut r: DagioReader<S> = dagio.load(link).await?;
        let mut buf = vec![];
        r.read_to_end(&mut buf).await?;
        Ok(buf)
    }
}
