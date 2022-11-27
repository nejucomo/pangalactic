use async_trait::async_trait;
use dagwasm_serialization::{AsyncDeserialize, AsyncSerialize};
use std::fmt::Debug;
use tokio::io::{AsyncRead, AsyncWrite};

#[async_trait]
pub trait BlobStore: Debug {
    type Key: Debug + AsyncSerialize + AsyncDeserialize + Send + Sync;
    type Reader: AsyncRead + Unpin + Send + Sync;
    type Writer: AsyncWrite + Unpin + Send + Sync;

    async fn open_reader(&mut self, key: Self::Key) -> anyhow::Result<Self::Reader>;
    async fn open_writer(&mut self) -> anyhow::Result<Self::Writer>;
    async fn commit_writer(&mut self, w: Self::Writer) -> anyhow::Result<Self::Key>;
}
