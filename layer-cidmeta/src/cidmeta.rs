use async_trait::async_trait;
use pangalactic_serialization::{AsyncDeserialize, AsyncSerialize};
use pangalactic_store::Store;
use std::fmt;
use std::marker::Unpin;
use std::str::FromStr;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug)]
pub struct CidMeta<S>
where
    S: Store,
{
    pub(crate) cid: <S as Store>::CID,
    pub(crate) node_size: u64,
}

impl<S> CidMeta<S>
where
    S: Store,
{
    pub fn node_size(&self) -> u64 {
        self.node_size
    }
}

impl<S> PartialEq for CidMeta<S>
where
    S: Store,
{
    fn eq(&self, other: &Self) -> bool {
        (self.cid == other.cid) && (self.node_size == other.node_size)
    }
}

impl<S> Eq for CidMeta<S> where S: Store {}

impl<S> Clone for CidMeta<S>
where
    S: Store,
{
    fn clone(&self) -> Self {
        CidMeta {
            cid: self.cid.clone(),
            node_size: self.node_size,
        }
    }
}

impl<S> FromStr for CidMeta<S>
where
    S: Store,
    <S as Store>::CID: FromStr<Err = anyhow::Error>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (cidtext, sizetext) = s
            .split_once(':')
            .ok_or_else(|| anyhow::anyhow!("missing ':' in {s:?}"))?;
        let cid = cidtext.parse()?;
        let node_size = sizetext.parse()?;
        Ok(CidMeta { cid, node_size })
    }
}

impl<S> fmt::Display for CidMeta<S>
where
    S: Store,
    <S as Store>::CID: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.cid.fmt(f)?;
        ':'.fmt(f)?;
        self.node_size.fmt(f)?;
        Ok(())
    }
}

#[async_trait]
impl<S> AsyncSerialize for CidMeta<S>
where
    S: Store,
{
    async fn write_into<W>(&self, mut w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        self.cid.write_into(&mut w).await?;
        self.node_size.write_into(&mut w).await?;
        Ok(())
    }
}

#[async_trait]
impl<S> AsyncDeserialize for CidMeta<S>
where
    S: Store,
{
    async fn read_from<R>(mut r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        let cid = <S as Store>::CID::read_from(&mut r).await?;
        let node_size = u64::read_from(&mut r).await?;
        Ok(CidMeta { cid, node_size })
    }
}
