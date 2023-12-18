use crate::Link;
use async_trait::async_trait;
use pangalactic_linkkind::LinkKind;
use pangalactic_serialization::{AsyncDeserialize, AsyncSerialize};
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

#[async_trait]
impl<K> AsyncSerialize for Link<K>
where
    K: AsyncSerialize + Send + Sync,
{
    async fn write_into<W>(&self, mut w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        self.kind().write_into(&mut w).await?;
        self.peek_key().write_into(&mut w).await?;
        Ok(())
    }
}

#[async_trait]
impl<K> AsyncDeserialize for Link<K>
where
    K: AsyncDeserialize + Send + Sync,
{
    async fn read_from<R>(mut r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        let kind = LinkKind::read_from(&mut r).await?;
        let key = K::read_from(&mut r).await?;
        Ok(Link::new(kind, key))
    }
}
