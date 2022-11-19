use crate::flexint::FlexIntEncoding;
use async_trait::async_trait;
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

#[async_trait]
pub trait AsyncSerialize {
    async fn write_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send;
}

#[async_trait]
pub trait AsyncDeserialize: Sized {
    async fn read_from<R>(r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send;
}

#[async_trait]
impl AsyncSerialize for u64 {
    async fn write_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        FlexIntEncoding::from(*self).write_into(w).await
    }
}

#[async_trait]
impl AsyncDeserialize for u64 {
    async fn read_from<R>(r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        let fie = FlexIntEncoding::read_from(r).await?;
        let u = u64::try_from(fie)?;
        Ok(u)
    }
}

#[async_trait]
impl AsyncSerialize for usize {
    async fn write_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        let u = u64::try_from(*self)?;
        u.write_into(w).await
    }
}

#[async_trait]
impl AsyncDeserialize for usize {
    async fn read_from<R>(r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        let u = u64::read_from(r).await?;
        let us = usize::try_from(u)?;
        Ok(us)
    }
}

#[async_trait]
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

#[async_trait]
impl AsyncDeserialize for Vec<u8> {
    async fn read_from<R>(mut r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        use tokio::io::AsyncReadExt;

        let bytelen = usize::read_from(&mut r).await?;
        // SECURITY BUG: we alloc on a size sent over the wire:
        let mut bytes = Vec::with_capacity(bytelen);
        r.read_exact(bytes.as_mut_slice()).await?;
        Ok(bytes)
    }
}

#[async_trait]
impl AsyncSerialize for str {
    async fn write_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        self.as_bytes().write_into(w).await
    }
}

#[async_trait]
impl AsyncDeserialize for String {
    async fn read_from<R>(r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        let bytes = Vec::<u8>::read_from(r).await?;
        let s = String::from_utf8(bytes)?;
        Ok(s)
    }
}

#[cfg(test)]
mod tests;
