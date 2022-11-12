use crate::Writer;
use async_trait::async_trait;
use std::fmt::Debug;
use tokio::io::AsyncRead;

#[async_trait]
pub trait BlobStore: Debug {
    type Link: Debug;
    type Reader: AsyncRead;
    type Writer: Writer<Link = Self::Link>;

    async fn open_reader(&mut self, link: Self::Link) -> anyhow::Result<Self::Reader>;
    async fn open_writer(&mut self) -> Self::Writer;
}
