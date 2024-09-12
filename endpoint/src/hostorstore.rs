use std::{
    fmt::{self, Display},
    future::Future,
    str::FromStr,
};

use anyhow::Result;
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

use crate::HostPath;

use self::HostOrStore::*;

#[pin_project(project=HosProjection)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HostOrStore<H, S> {
    MkHost(#[pin] H),
    MkStore(#[pin] S),
}

impl<H, S> HostOrStore<H, S> {
    pub fn as_ref(&self) -> HostOrStore<&H, &S> {
        match self {
            MkHost(x) => MkHost(x),
            MkStore(x) => MkStore(x),
        }
    }

    pub fn map_host<F, H2>(self, f: F) -> HostOrStore<H2, S>
    where
        F: FnOnce(H) -> H2,
    {
        match self {
            MkHost(h) => MkHost(f(h)),
            MkStore(s) => MkStore(s),
        }
    }

    pub fn map_store<F, S2>(self, f: F) -> HostOrStore<H, S2>
    where
        F: FnOnce(S) -> S2,
    {
        match self {
            MkHost(h) => MkHost(h),
            MkStore(s) => MkStore(f(s)),
        }
    }

    pub fn map_either_with<A, FH, RH, FS, RS>(
        self,
        arg: A,
        map_host: FH,
        map_store: FS,
    ) -> HostOrStore<RH, RS>
    where
        FH: FnOnce(H, A) -> RH,
        FS: FnOnce(S, A) -> RS,
    {
        self.project_into_with(
            arg,
            |h, a| MkHost(map_host(h, a)),
            |s, a| MkStore(map_store(s, a)),
        )
    }

    pub fn project_into_with<A, FH, FS, R>(self, arg: A, host_into: FH, store_into: FS) -> R
    where
        FH: FnOnce(H, A) -> R,
        FS: FnOnce(S, A) -> R,
    {
        match self {
            MkHost(h) => host_into(h, arg),
            MkStore(s) => store_into(s, arg),
        }
    }

    pub async fn await_futures(self) -> HostOrStore<<H as Future>::Output, <S as Future>::Output>
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

impl<T> HostOrStore<T, T> {
    pub fn distill(self) -> T {
        match self {
            MkHost(t) => t,
            MkStore(t) => t,
        }
    }
}

impl<H, S, E> HostOrStore<Result<H, E>, Result<S, E>> {
    pub fn transpose(self) -> Result<HostOrStore<H, S>, E> {
        match self {
            MkHost(h) => h.map(MkHost),
            MkStore(s) => s.map(MkStore),
        }
    }
}

impl<H, S> AsyncRead for HostOrStore<H, S>
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

impl<S> IntoSource<S> for HostOrStore<HostPath, Link<S::CID>>
where
    S: Store,
{
    type Leaf = HostOrStore<File, <LinkDirectoryLayer<S> as Store>::Reader>;
    type Branch = HostOrStore<ReadDir, DirectoryIntoIter<Link<S::CID>>>;

    async fn into_source(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Source<Self::Leaf, Self::Branch>> {
        tracing::debug!("loading origin {}", &self);
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

impl<S> BranchIter<S> for HostOrStore<ReadDir, DirectoryIntoIter<Link<S::CID>>>
where
    S: Store,
{
    type IntoSource = HostOrStore<HostPath, Link<S::CID>>;

    async fn next_branch_entry(&mut self) -> anyhow::Result<Option<(Name, Self::IntoSource)>> {
        match self {
            MkHost(x) => <ReadDir as BranchIter<S>>::next_branch_entry(x)
                .await
                .map_branch_item(|p| MkHost(HostPath::from(p))),
            MkStore(x) => <DirectoryIntoIter<Link<S::CID>> as BranchIter<S>>::next_branch_entry(x)
                .await
                .map_branch_item(MkStore),
        }
    }
}

impl<S> Commit<LinkDirectoryLayer<S>> for HostOrStore<ReadDir, DirectoryIntoIter<Link<S::CID>>>
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

impl<H, S> Display for HostOrStore<H, S>
where
    H: Display,
    S: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref()
            .map_host(|pb| pb.fmt(f))
            .map_store(|sp| sp.fmt(f))
            .transpose()
            .map(HostOrStore::distill)
    }
}

impl<H, S> FromStr for HostOrStore<H, S>
where
    H: FromStr<Err = anyhow::Error>,
    S: FromStr<Err = anyhow::Error>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with(pangalactic_link::SCHEME_PREFIX) {
            s.parse().map(MkStore)
        } else {
            s.parse().map(MkHost)
        }
    }
}
