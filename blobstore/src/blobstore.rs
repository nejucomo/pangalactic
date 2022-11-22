use async_trait::async_trait;
use std::fmt::Debug;
use tokio::io::{AsyncRead, AsyncWrite};

#[async_trait]
pub trait BlobStore: Debug {
    type Key: Debug;
    type Reader: AsyncRead;
    type Writer: AsyncWrite;

    async fn open_reader(&mut self, key: Self::Key) -> anyhow::Result<Self::Reader>;
    async fn open_writer(&mut self) -> anyhow::Result<Self::Writer>;
    async fn commit_writer(&mut self, w: Self::Writer) -> anyhow::Result<Self::Key>;
}
