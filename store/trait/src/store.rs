use crate::StoreCid;
use async_trait::async_trait;
use std::fmt::Debug;
use tokio::io::{AsyncRead, AsyncWrite};

// Documentation readability hack; see https://github.com/dtolnay/async-trait/issues/213#issuecomment-1559690487
#[cfg_attr(doc, feature(async_fn_in_trait))]
#[cfg_attr(not(doc), async_trait)]
pub trait Store: Debug + Send {
    type Cid: StoreCid;
    type Reader: AsyncRead + Unpin + Send + Sync;
    type Writer: AsyncWrite + Unpin + Send + Sync;

    async fn open_reader(&mut self, key: &Self::Cid) -> anyhow::Result<Self::Reader>;
    async fn open_writer(&mut self) -> anyhow::Result<Self::Writer>;
    async fn commit_writer(&mut self, w: Self::Writer) -> anyhow::Result<Self::Cid>;

    async fn read(&mut self, key: &Self::Cid) -> anyhow::Result<Vec<u8>> {
        use tokio::io::AsyncReadExt;

        let mut buf = vec![];
        let mut r = self.open_reader(key).await?;
        r.read_to_end(&mut buf).await?;
        Ok(buf)
    }

    async fn write(&mut self, contents: &[u8]) -> anyhow::Result<Self::Cid> {
        use tokio::io::AsyncWriteExt;

        let mut w = self.open_writer().await?;
        w.write_all(contents).await?;
        self.commit_writer(w).await
    }
}
