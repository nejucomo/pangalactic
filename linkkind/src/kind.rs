use async_trait::async_trait;
use dagwasm_serialization::{AsyncDeserialize, AsyncSerialize};
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LinkKind {
    File,
    Dir,
}

impl TryFrom<u64> for LinkKind {
    type Error = String;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        use LinkKind::*;

        match u {
            0 => Ok(File),
            1 => Ok(Dir),
            _ => Err(format!("invalid LinkKind encoding {u:?}")),
        }
    }
}

impl From<LinkKind> for u64 {
    fn from(lk: LinkKind) -> u64 {
        use LinkKind::*;

        match lk {
            File => 0,
            Dir => 1,
        }
    }
}

#[async_trait]
impl AsyncSerialize for LinkKind {
    async fn write_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        u64::from(*self).write_into(w).await
    }
}

#[async_trait]
impl AsyncDeserialize for LinkKind {
    async fn read_from<R>(r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        let encoding = u64::read_from(r).await?;
        LinkKind::try_from(encoding).map_err(anyhow::Error::msg)
    }
}
