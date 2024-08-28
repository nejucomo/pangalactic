use std::future::Future;

use anyhow::Result;
use pangalactic_asynctryiter::{AsyncTryIterator, IntoAsyncTryIterator};
use pangalactic_iowrappers::Readable;
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_name::Name;
use pangalactic_store::{Commit, Store};
use tokio::io::AsyncRead;

use crate::{BranchSource, IntoSource, LeafSource, Sink};

impl<I, T, S> Sink<BranchSource<I, T>> for LinkDirectoryLayer<S>
where
    I: IntoAsyncTryIterator<Item = (Name, T)> + Send,
    T: IntoSource,
    S: Store,
    LinkDirectoryLayer<S>: Sink<T::Source>,
{
    type CID = Link<S::CID>;

    async fn sink(mut self, source: BranchSource<I, T>) -> Result<Self::CID> {
        self.commit(source).await
    }
}

impl<I, T, S> Commit<LinkDirectoryLayer<S>> for BranchSource<I, T>
where
    I: IntoAsyncTryIterator<Item = (Name, T)> + Send,
    T: IntoSource,
    S: Store,
    LinkDirectoryLayer<S>: Sink<T::Source>,
{
    async fn commit_into_store(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>> {
        let mut it = self.into_async_try_iter();
        let mut ld = LinkDirectory::default();
        while let Some((name, subinto)) = it.try_next_async().await? {
            let subsrc = subinto.into_source().await?;
            let sublink = Box::pin(store.sink(subsrc)).await?;
            ld.insert(name, sublink)?;
        }

        store.commit(ld).await
    }
}

impl<R, S> Sink<LeafSource<R>> for LinkDirectoryLayer<S>
where
    R: AsyncRead + Send,
    S: Store,
{
    type CID = Link<S::CID>;

    async fn sink(mut self, source: LeafSource<R>) -> Result<Self::CID> {
        self.commit(source).await
    }
}

impl<R, S> Commit<LinkDirectoryLayer<S>> for LeafSource<R>
where
    R: AsyncRead + Send,
    S: Store,
{
    fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<<LinkDirectoryLayer<S> as Store>::CID>> + Send {
        Readable(self.0).commit_into_store(store)
    }
}
