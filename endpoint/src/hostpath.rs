use std::{
    fmt,
    future::Future,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Result;
use futures::FutureExt;
use pangalactic_dag_transfer::{BranchIter, Destination, IntoSource, LeafDestination, Source};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_store::Store;
use tokio::{
    fs::{File, ReadDir},
    io::AsyncRead,
};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct HostPath(PathBuf);

impl HostPath {
    pub fn unwrap(self) -> PathBuf {
        self.into()
    }

    pub fn join<P>(&self, other: P) -> Self
    where
        P: AsRef<Path>,
    {
        HostPath(self.0.join(other))
    }
}

impl<S> IntoSource<S> for HostPath
where
    S: Store,
{
    type Leaf = File;
    type Branch = ReadDir;

    fn into_source(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<Self::Leaf, Self::Branch>>> + Send {
        self.0.into_source(store)
    }
}

impl<S> Destination<S> for HostPath
where
    S: Store,
{
    fn sink_branch<B>(
        self,
        store: &mut LinkDirectoryLayer<S>,
        branch: B,
    ) -> impl Future<Output = Result<Self::CID>> + Send
    where
        B: fmt::Debug + Send + BranchIter<S>,
    {
        self.0.sink_branch(store, branch).map(|res| res.map(Self))
    }
}

impl<S> LeafDestination<S> for HostPath
where
    S: Store,
{
    type CID = Self;

    fn sink_leaf<L>(
        self,
        store: &mut LinkDirectoryLayer<S>,
        leaf: L,
    ) -> impl Future<Output = Result<Self::CID>> + Send
    where
        L: fmt::Debug + Send + AsyncRead,
    {
        self.0.sink_leaf(store, leaf).map(|res| res.map(Self))
    }
}

impl AsRef<Path> for HostPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl fmt::Display for HostPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.display().fmt(f)
    }
}

impl FromStr for HostPath {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        s.parse().map(HostPath).map_err(anyhow::Error::from)
    }
}
