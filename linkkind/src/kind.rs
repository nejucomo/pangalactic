use async_trait::async_trait;
use pangalactic_primitives::{self as prim, LINK_KIND_DIR, LINK_KIND_FILE};
use pangalactic_serialization::{AsyncDeserialize, AsyncSerialize};
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LinkKind {
    File,
    Dir,
}

impl TryFrom<prim::LinkKind> for LinkKind {
    type Error = String;

    fn try_from(u: prim::LinkKind) -> Result<Self, Self::Error> {
        use LinkKind::*;

        match u {
            LINK_KIND_FILE => Ok(File),
            LINK_KIND_DIR => Ok(Dir),
            _ => Err(format!("invalid LinkKind encoding {u:?}")),
        }
    }
}

impl From<LinkKind> for prim::LinkKind {
    fn from(lk: LinkKind) -> prim::LinkKind {
        use LinkKind::*;

        match lk {
            File => LINK_KIND_FILE,
            Dir => LINK_KIND_DIR,
        }
    }
}

#[async_trait]
impl AsyncSerialize for LinkKind {
    async fn write_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        prim::LinkKind::from(*self).write_into(w).await
    }
}

#[async_trait]
impl AsyncDeserialize for LinkKind {
    async fn read_from<R>(r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        let encoding = prim::LinkKind::read_from(r).await?;
        LinkKind::try_from(encoding).map_err(anyhow::Error::msg)
    }
}
