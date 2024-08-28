use std::future::Future;

use anyhow::Result;
use pangalactic_iowrappers::Readable;
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::{Commit, Store};
use tokio::io::AsyncRead;

use crate::{BranchIter, BranchSource, IntoSource, LeafSource, Sink};

impl<B, S> Sink<BranchSource<B>> for LinkDirectoryLayer<S>
where
    B: BranchIter,
    S: Store,
    LinkDirectoryLayer<S>: Sink<<B::IntoSource as IntoSource>::Source>,
{
    type CID = Link<S::CID>;

    async fn sink(mut self, source: BranchSource<B>) -> Result<Self::CID> {
        self.commit(source).await
    }
}

impl<B, S> Commit<LinkDirectoryLayer<S>> for BranchSource<B>
where
    B: BranchIter,
    S: Store,
    LinkDirectoryLayer<S>: Sink<<B::IntoSource as IntoSource>::Source, CID = Link<S::CID>>,
{
    async fn commit_into_store(
        mut self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> Result<Link<S::CID>> {
        let mut ld = LinkDirectory::default();
        while let Some((name, subinto)) = self.next_branch_entry().await? {
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
