use crate::codec::{AsyncDeserialize, AsyncSerialize};
use async_trait::async_trait;
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, PartialEq, Eq)]
pub enum LinkKind {
    File,
    Dir,
}

#[async_trait]
impl AsyncSerialize for LinkKind {
    async fn write_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        use LinkKind::*;

        let encoding: u64 = match self {
            File => 0,
            Dir => 1,
        };

        encoding.write_into(w).await
    }
}

#[async_trait]
impl AsyncDeserialize for LinkKind {
    async fn read_from<R>(r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        use LinkKind::*;

        let encoding = u64::read_from(r).await?;
        match encoding {
            0 => Ok(File),
            1 => Ok(Dir),
            other => Err(anyhow::Error::msg(format!(
                "invalid Link encoding {:?}",
                other
            ))),
        }
    }
}
