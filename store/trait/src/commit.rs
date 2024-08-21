use std::{future::Future, pin::pin};

use anyhow::Result;
use pangalactic_iowrappers::Readable;
use tokio::io::AsyncRead;

use crate::Store;

pub trait Commit<S>
where
    S: Store,
{
    fn commit_into_store(self, store: &mut S) -> impl Future<Output = Result<S::CID>> + Send;
}

impl<S, T> Commit<S> for Box<T>
where
    S: Store,
    T: Commit<S> + Send,
{
    async fn commit_into_store(self, store: &mut S) -> Result<S::CID> {
        store.commit(*self).await
    }
}

impl<S, R> Commit<S> for Readable<R>
where
    S: Store,
    R: AsyncRead + Send,
{
    async fn commit_into_store(self, store: &mut S) -> Result<S::CID> {
        let mut w = store.open_writer().await?;
        tokio::io::copy(&mut pin!(self), &mut w).await?;
        store.commit(w).await
    }
}

impl<'a, S> Commit<S> for &'a [u8]
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut S) -> Result<S::CID> {
        store.commit(Readable(self)).await
    }
}

impl<S> Commit<S> for Vec<u8>
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut S) -> Result<S::CID> {
        store.commit(self.as_slice()).await
    }
}
