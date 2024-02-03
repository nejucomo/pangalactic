use crate::StoreCid;
use async_trait::async_trait;
use std::fmt::Debug;
use tokio::io::{AsyncRead, AsyncWrite};

// Documentation readability hack; see https://github.com/dtolnay/async-trait/issues/213#issuecomment-1559690487
#[cfg_attr(doc, feature(async_fn_in_trait))]
#[cfg_attr(not(doc), async_trait)]
pub trait Store: Debug + Send + Sync {
    /// A unique identifying string for this store type
    const TAG: &'static str;

    /// An acronym for `Content IDentifier` required to have these properties beyond the type
    /// signature:
    ///
    /// - Inserting the same bytes sequence into a store multiple times produces the same `CID` on
    ///   any host.
    /// - Two distinct byte sequences never produce the same `CID` upon insertion into the store on
    ///   any host.
    /// - A `CID` should be concise.
    ///
    /// Cryptographic hash functions over the content are assumed to meet these properties.
    type CID: StoreCid;
    type Reader: AsyncRead + Unpin + Send + Sync;
    type Writer: AsyncWrite + Unpin + Send + Sync;

    async fn open_reader(&self, key: &Self::CID) -> anyhow::Result<Self::Reader>;
    async fn open_writer(&self) -> anyhow::Result<Self::Writer>;
    async fn commit_writer(&mut self, w: Self::Writer) -> anyhow::Result<Self::CID>;

    async fn read(&self, key: &Self::CID) -> anyhow::Result<Vec<u8>> {
        use tokio::io::AsyncReadExt;

        let mut buf = vec![];
        let mut r = self.open_reader(key).await?;
        r.read_to_end(&mut buf).await?;
        Ok(buf)
    }

    async fn write(&mut self, contents: &[u8]) -> anyhow::Result<Self::CID> {
        use tokio::io::AsyncWriteExt;

        let mut w = self.open_writer().await?;
        w.write_all(contents).await?;
        self.commit_writer(w).await
    }
}
