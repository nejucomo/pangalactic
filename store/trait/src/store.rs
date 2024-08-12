use std::{fmt::Debug, future::Future};

use anyhow::Result;
use pangalactic_cid::ContentIdentifier;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::{Commit, Load};

pub trait Store: Sized + Debug + Send + Sync {
    type CID: ContentIdentifier;
    type Reader: Load<Self> + AsyncRead + Unpin + Send + Sync;
    type Writer: Commit<Self> + AsyncWrite + Unpin + Send + Sync;

    /// Callers typically use these:
    fn commit<T>(&mut self, object: T) -> impl Future<Output = Result<Self::CID>> + Send
    where
        T: Commit<Self> + Send,
    {
        object.commit_into_store(self)
    }

    fn load<T>(&self, cid: &Self::CID) -> impl Future<Output = Result<T>> + Send
    where
        T: Load<Self>,
    {
        T::load_from_store(self, cid)
    }

    /// Implementors must provide these:
    // TODO: Move to a distinct inherited trait, eg `StoreProvider`?
    fn open_writer(&self) -> impl Future<Output = Result<Self::Writer>> + Send;
}
