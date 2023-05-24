use crate::flexint::FlexIntEncoding;
use async_trait::async_trait;
use std::marker::Unpin;
use tokio::io::AsyncRead;

#[cfg_attr(not(doc), async_trait)]
pub trait AsyncDeserialize: Sized {
    async fn read_from<R>(r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send;
}

#[cfg_attr(not(doc), async_trait)]
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

#[cfg_attr(not(doc), async_trait)]
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

#[cfg_attr(not(doc), async_trait)]
impl<const K: usize> AsyncDeserialize for [u8; K] {
    async fn read_from<R>(mut r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        use tokio::io::AsyncReadExt;

        let mut buf = [0; K];
        r.read_exact(&mut buf[..]).await?;
        Ok(buf)
    }
}

#[cfg_attr(not(doc), async_trait)]
impl AsyncDeserialize for Vec<u8> {
    async fn read_from<R>(mut r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        use tokio::io::AsyncReadExt;

        let bytelen = usize::read_from(&mut r).await?;
        // SECURITY BUG: we alloc on a size sent over the wire:
        let mut bytes = vec![0u8; bytelen];
        r.read_exact(bytes.as_mut_slice()).await?;
        Ok(bytes)
    }
}

#[cfg_attr(not(doc), async_trait)]
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
