use crate::Writer;
use async_trait::async_trait;
use std::fmt::Debug;
use tokio::io::AsyncRead;

#[async_trait]
pub trait BlobStore: Debug {
    type Key: Debug;
    type Reader: AsyncRead;
    type Writer: Writer<Key = Self::Key>;

    async fn open_reader(&mut self, link: Self::Key) -> anyhow::Result<Self::Reader>;
    async fn open_writer(&mut self) -> anyhow::Result<Self::Writer>;
}
