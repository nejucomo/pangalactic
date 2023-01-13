use async_trait::async_trait;
use dagwasm_linkkind::LinkKind;
use dagwasm_serialization::{AsyncDeserialize, AsyncSerialize};
use std::fmt::Debug;
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link<K> {
    kind: LinkKind,
    key: K,
}

impl<K> Link<K> {
    pub fn new(kind: LinkKind, key: K) -> Self {
        Link { kind, key }
    }

    pub fn kind(&self) -> LinkKind {
        self.kind
    }

    pub fn peek_key(&self, kind: LinkKind) -> anyhow::Result<&K>
    where
        K: Debug,
    {
        if self.kind == kind {
            Ok(&self.key)
        } else {
            Err(anyhow::Error::msg(format!(
                "expected link kind {:?}, found {:?}",
                kind, self.kind
            )))
        }
    }

    pub fn unwrap(self) -> (LinkKind, K) {
        (self.kind, self.key)
    }
}

#[async_trait]
impl<K> AsyncSerialize for Link<K>
where
    K: AsyncSerialize + Send + Sync,
{
    async fn write_into<W>(&self, mut w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        self.kind.write_into(&mut w).await?;
        self.key.write_into(&mut w).await?;
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
        Ok(Link { kind, key })
    }
}
