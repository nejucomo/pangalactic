use crate::dag::Writer;
use async_trait::async_trait;

#[async_trait]
pub trait Store {
    type Link;
    type Reader: tokio::io::AsyncRead;
    type Writer: Writer<Link = Self::Link>;

    async fn open_reader(&mut self, link: Self::Link) -> anyhow::Result<Self::Reader>;
    async fn open_writer(&mut self) -> Self::Writer;
}
