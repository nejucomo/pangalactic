use std::fmt::Debug;

use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::{Commit, Load, StoreCid};

// Documentation readability hack; see https://github.com/dtolnay/async-trait/issues/213#issuecomment-1559690487
#[cfg_attr(doc, feature(async_fn_in_trait))]
#[cfg_attr(not(doc), async_trait)]
pub trait Store: Sized + Debug + Send + Sync {
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
    type Reader: Load<Self> + AsyncRead + Unpin + Send + Sync;
    type Writer: Commit<Self> + AsyncWrite + Unpin + Send + Sync;

    /// Callers typically use these:
    async fn commit<T>(&mut self, object: T) -> anyhow::Result<Self::CID>
    where
        T: Commit<Self> + Send,
    {
        object.commit_into_store(self).await
    }

    async fn load<T>(&self, cid: &Self::CID) -> anyhow::Result<T>
    where
        T: Load<Self>,
    {
        T::load_from_store(self, cid).await
    }

    /// Implementors must provide these:
    async fn open_writer(&self) -> anyhow::Result<Self::Writer>;
}
