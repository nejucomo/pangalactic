use crate::flexint::FlexIntEncoding;
use async_trait::async_trait;
use std::marker::Unpin;
use tokio::io::AsyncWrite;

#[cfg_attr(not(doc), async_trait)]
pub trait AsyncSerialize {
    async fn write_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send;
}

#[cfg_attr(not(doc), async_trait)]
impl AsyncSerialize for u64 {
    async fn write_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        FlexIntEncoding::from(*self).write_into(w).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl AsyncSerialize for usize {
    async fn write_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        let u = u64::try_from(*self)?;
        u.write_into(w).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl AsyncSerialize for [u8] {
    async fn write_into<W>(&self, mut w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        use tokio::io::AsyncWriteExt;

        self.len().write_into(&mut w).await?;
        w.write_all(self).await?;
        Ok(())
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<const K: usize> AsyncSerialize for [u8; K] {
    async fn write_into<W>(&self, mut w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        use tokio::io::AsyncWriteExt;

        w.write_all(&self[..]).await?;
        Ok(())
    }
}

#[cfg_attr(not(doc), async_trait)]
impl AsyncSerialize for str {
    async fn write_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        self.as_bytes().write_into(w).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl AsyncSerialize for String {
    async fn write_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        self.as_str().write_into(w).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl AsyncSerialize for Vec<u8> {
    async fn write_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        self.as_slice().write_into(w).await
    }
}
