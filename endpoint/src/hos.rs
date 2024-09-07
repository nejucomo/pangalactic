use std::{future::Future, path::PathBuf};

use pangalactic_dag_transfer::{BranchIter, BranchIterOutput, IntoSource, Source};
use pangalactic_layer_dir::{DirectoryIntoIter, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_name::Name;
use pangalactic_store::{Commit, Store};
use pin_project::pin_project;
use tokio::{
    fs::{File, ReadDir},
    io::AsyncRead,
};

use self::Hos::*;

#[pin_project(project=HosProjection)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Hos<H, S> {
    MkHost(#[pin] H),
    MkStore(#[pin] S),
}

impl<H, S> Hos<H, S> {
    pub fn as_ref(&self) -> Hos<&H, &S> {
        match self {
            MkHost(x) => MkHost(x),
            MkStore(x) => MkStore(x),
        }
    }

    pub fn map_host<F, H2>(self, f: F) -> Hos<H2, S>
    where
        F: FnOnce(H) -> H2,
    {
        match self {
            MkHost(h) => MkHost(f(h)),
            MkStore(s) => MkStore(s),
        }
    }

    pub fn map_store<F, S2>(self, f: F) -> Hos<H, S2>
    where
        F: FnOnce(S) -> S2,
    {
        match self {
            MkHost(h) => MkHost(h),
            MkStore(s) => MkStore(f(s)),
        }
    }

    pub fn map_into<FH, FS, T>(self, host_into: FH, store_into: FS) -> T
    where
        FH: FnOnce(H) -> T,
        FS: FnOnce(S) -> T,
    {
        match self {
            MkHost(h) => host_into(h),
            MkStore(s) => store_into(s),
        }
    }

    pub async fn await_futures(self) -> Hos<<H as Future>::Output, <S as Future>::Output>
    where
        H: Future,
        S: Future,
    {
        match self {
            MkHost(h) => MkHost(h.await),
            MkStore(s) => MkStore(s.await),
        }
    }
}

impl<T> Hos<T, T> {
    pub fn distill(self) -> T {
        match self {
            MkHost(t) => t,
            MkStore(t) => t,
        }
    }
}

impl<H, S, E> Hos<Result<H, E>, Result<S, E>> {
    pub fn transpose(self) -> Result<Hos<H, S>, E> {
        match self {
            MkHost(h) => h.map(MkHost),
            MkStore(s) => s.map(MkStore),
        }
    }
}

impl<H, S> AsyncRead for Hos<H, S>
where
    H: AsyncRead,
    S: AsyncRead,
{
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        use HosProjection::*;

        match self.project() {
            MkHost(x) => x.poll_read(cx, buf),
            MkStore(x) => x.poll_read(cx, buf),
        }
    }
}

impl<S> IntoSource<S> for Hos<PathBuf, Link<S::CID>>
where
    S: Store,
{
    type Leaf = Hos<File, <LinkDirectoryLayer<S> as Store>::Reader>;
    type Branch = Hos<ReadDir, DirectoryIntoIter<Link<S::CID>>>;

    async fn into_source(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Source<Self::Leaf, Self::Branch>> {
        match self {
            MkHost(x) => x
                .into_source(store)
                .await
                .map(|x| x.map_leaf(MkHost).map_branch(MkHost)),
            MkStore(x) => x
                .into_source(store)
                .await
                .map(|x| x.map_leaf(MkStore).map_branch(MkStore)),
        }
    }
}

impl<S> BranchIter<S> for Hos<ReadDir, DirectoryIntoIter<Link<S::CID>>>
where
    S: Store,
{
    type IntoSource = Hos<PathBuf, Link<S::CID>>;

    async fn next_branch_entry(&mut self) -> anyhow::Result<Option<(Name, Self::IntoSource)>> {
        match self {
            MkHost(x) => <ReadDir as BranchIter<S>>::next_branch_entry(x)
                .await
                .map_branch_item(MkHost),
            MkStore(x) => <DirectoryIntoIter<Link<S::CID>> as BranchIter<S>>::next_branch_entry(x)
                .await
                .map_branch_item(MkStore),
        }
    }
}

impl<S> Commit<LinkDirectoryLayer<S>> for Hos<ReadDir, DirectoryIntoIter<Link<S::CID>>>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        match self {
            MkHost(x) => x.commit_into_store(store).await,
            MkStore(x) => x.commit_into_store(store).await,
        }
    }
}
