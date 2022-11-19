use crate::codec::{AsyncDeserialize, AsyncSerialize};
use async_trait::async_trait;
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

const MAX_SIZE: usize = 10;

/// Provides encoding/decoding U64 in a flex-int format
#[derive(Clone, Debug)]
pub(crate) struct FlexIntEncoding {
    buf: [u8; MAX_SIZE],
    used: usize,
}

impl FlexIntEncoding {
    pub(crate) fn as_slice(&self) -> &[u8] {
        assert!(self.used <= MAX_SIZE);
        &self.buf[..self.used]
    }

    fn empty() -> Self {
        FlexIntEncoding {
            buf: [0; MAX_SIZE],
            used: 0,
        }
    }

    fn check_overflow(&self) -> anyhow::Result<()> {
        u64::try_from(self).map(|_| ())
    }
}

#[async_trait]
impl AsyncSerialize for FlexIntEncoding {
    async fn write_into<W>(&self, mut w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        use tokio::io::AsyncWriteExt;

        w.write_all(self.as_slice()).await?;
        Ok(())
    }
}

#[async_trait]
impl AsyncDeserialize for FlexIntEncoding {
    async fn read_from<R>(mut r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        use tokio::io::AsyncReadExt;

        let mut fie = FlexIntEncoding::empty();

        while fie.used == 0 || (fie.used < MAX_SIZE && high_bit_set(fie.buf[fie.used - 1])) {
            fie.buf[fie.used] = r.read_u8().await?;
            fie.used += 1;
        }

        fie.check_overflow()?;
        Ok(fie)
    }
}

impl From<u64> for FlexIntEncoding {
    fn from(mut u: u64) -> Self {
        let mut fie = FlexIntEncoding::empty();

        if u == 0 {
            fie.used = 1;
        } else {
            while u > 0 {
                let flagbit: u8 = if (u >> 7) > 0 { 0x80 } else { 0x00 };
                fie.buf[fie.used] = flagbit | ((u & 0x7f) as u8);
                fie.used += 1;
                u >>= 7;
            }
        }

        fie
    }
}

impl TryFrom<FlexIntEncoding> for u64 {
    type Error = anyhow::Error;

    fn try_from(fie: FlexIntEncoding) -> anyhow::Result<u64> {
        u64::try_from(&fie)
    }
}

impl<'a> TryFrom<&'a FlexIntEncoding> for u64 {
    type Error = anyhow::Error;

    fn try_from(fie: &'a FlexIntEncoding) -> anyhow::Result<u64> {
        let mut u: u64 = 0;

        let slice = fie.as_slice();
        for (i, &b) in slice.iter().enumerate() {
            if i + 1 == MAX_SIZE && b > 0x01 {
                return Err(anyhow::Error::msg(format!("overflow @{} {:?}", i, slice)));
            } else if i + 1 == slice.len() {
                if high_bit_set(b) {
                    return Err(anyhow::Error::msg(format!(
                        "unexpected continuation bit @{} {:?}",
                        i, slice
                    )));
                }
            } else if !high_bit_set(b) {
                return Err(anyhow::Error::msg(format!(
                    "missing continuation bit @{} {:?}",
                    i, slice
                )));
            }

            u |= ((b & 0x7f) as u64) << (i * 7);
        }

        Ok(u)
    }
}

impl TryFrom<&[u8]> for FlexIntEncoding {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> anyhow::Result<FlexIntEncoding> {
        let used = slice.len();
        if used <= MAX_SIZE {
            let mut buf = [0; MAX_SIZE];
            buf[..used].copy_from_slice(slice);
            let fie = FlexIntEncoding { buf, used };
            fie.check_overflow()?;
            Ok(fie)
        } else {
            Err(anyhow::Error::msg(format!(
                "byte encoding too long {} vs max size {}",
                slice.len(),
                MAX_SIZE
            )))
        }
    }
}

fn high_bit_set(b: u8) -> bool {
    b & 0x80 == 0x80
}

#[cfg(test)]
mod tests;
