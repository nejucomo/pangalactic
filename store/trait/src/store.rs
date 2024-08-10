use std::fmt::Debug;

use pangalactic_cid::ContentIdentifier;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::{Commit, Load};

pub trait Store: Sized + Debug + Send + Sync {
    type CID: ContentIdentifier;
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
    // TODO: Move to a distinct inherited trait, eg `StoreProvider`?
    async fn open_writer(&self) -> anyhow::Result<Self::Writer>;
}
