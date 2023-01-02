use async_trait::async_trait;
use dagwasm_serialization::{AsyncDeserialize, AsyncSerialize};
use std::fmt::Debug;
use tokio::io::{AsyncRead, AsyncWrite};

#[async_trait]
pub trait Store: Debug + Send {
    type Key: Clone + Debug + AsyncSerialize + AsyncDeserialize + Send + Sync;
    type Reader: AsyncRead + Unpin + Send + Sync;
    type Writer: AsyncWrite + Unpin + Send + Sync;

    async fn open_reader(&mut self, key: &Self::Key) -> anyhow::Result<Self::Reader>;
    async fn open_writer(&mut self) -> anyhow::Result<Self::Writer>;
    async fn commit_writer(&mut self, w: Self::Writer) -> anyhow::Result<Self::Key>;

    async fn read(&mut self, key: &Self::Key) -> anyhow::Result<Vec<u8>> {
        use tokio::io::AsyncReadExt;

        let mut buf = vec![];
        let mut r = self.open_reader(key).await?;
        r.read_to_end(&mut buf).await?;
        Ok(buf)
    }

    async fn write(&mut self, contents: &[u8]) -> anyhow::Result<Self::Key> {
        use tokio::io::AsyncWriteExt;

        let mut w = self.open_writer().await?;
        w.write_all(contents).await?;
        self.commit_writer(w).await
    }
}
