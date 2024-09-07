mod branch;
mod leaf;

use std::{fmt, future::ready, path::PathBuf, str::FromStr};

use anyhow::Result;
use pangalactic_dag_transfer::{
    IntoSource,
    Source::{self, Branch, Leaf},
};
use pangalactic_layer_dir::{DirectoryIntoIter, LinkDirectoryLayer};
use pangalactic_link::{Link, SCHEME_PREFIX};
use pangalactic_linkpath::LinkPath;
use pangalactic_store::Store;
use serde::{de::DeserializeOwned, Serialize};
use tokio::{
    fs::{File, ReadDir},
    io::Stdin,
};

use crate::{
    hos::Hos::{self, MkHost, MkStore},
    iohos::Iohos::{self, MkHos, MkStdio},
};

pub use self::{branch::SourceEndpointBranch, leaf::SourceEndpointLeaf};

#[derive(Clone, Eq, PartialEq, derive_more::From)]
pub struct SourceEndpoint<C>(Iohos<(), PathBuf, LinkPath<C>>);

type PathBufSource = Source<File, ReadDir>;
type LinkPathSource<S> = Source<LinkPathLeaf<S>, LinkPathBranch<S>>;
type LinkPathLeaf<S> = <LinkDirectoryLayer<S> as Store>::Reader;
type LinkPathBranch<S> = DirectoryIntoIter<Link<<S as Store>::CID>>;

impl<C> SourceEndpoint<C> {
    pub fn mk_stdin() -> Self {
        ().into()
    }
}

async fn into_source_endpoints<S>(
    endpoint: SourceEndpoint<S::CID>,
    store: &LinkDirectoryLayer<S>,
) -> Result<Iohos<Stdin, PathBufSource, LinkPathSource<S>>>
where
    S: Store,
{
    endpoint
        .0
        .map_io(|()| ready(Ok(tokio::io::stdin())))
        .map_host(|p| p.into_source(store))
        .map_store(|p| p.into_source(store))
        .await_futures()
        .await
        .transpose()
}

impl<S> IntoSource<S> for SourceEndpoint<S::CID>
where
    S: Store,
{
    type Leaf = SourceEndpointLeaf<S>;
    type Branch = SourceEndpointBranch<S>;

    async fn into_source(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> Result<Source<Self::Leaf, Self::Branch>> {
        let seps = into_source_endpoints(self, store).await?;
        Ok(seps.map_into(
            |stdin| Leaf(SourceEndpointLeaf(MkStdio(stdin))),
            |hostsrc| {
                hostsrc.map_into(
                    |l| Leaf(SourceEndpointLeaf(MkHos(MkHost(l)))),
                    |b| Branch(SourceEndpointBranch::from(MkHost(b))),
                )
            },
            |storesrc| {
                storesrc.map_into(
                    |l| Leaf(SourceEndpointLeaf(MkHos(MkStore(l)))),
                    |b| Branch(SourceEndpointBranch::from(MkStore(b))),
                )
            },
        ))
    }
}

impl<C> From<()> for SourceEndpoint<C> {
    fn from((): ()) -> Self {
        SourceEndpoint(MkStdio(()))
    }
}

impl<C> From<PathBuf> for SourceEndpoint<C> {
    fn from(path: PathBuf) -> Self {
        SourceEndpoint(MkHos(MkHost(path)))
    }
}

impl<C> From<LinkPath<C>> for SourceEndpoint<C> {
    fn from(path: LinkPath<C>) -> Self {
        SourceEndpoint(MkHos(MkStore(path)))
    }
}

impl<C> From<Hos<PathBuf, LinkPath<C>>> for SourceEndpoint<C> {
    fn from(hos: Hos<PathBuf, LinkPath<C>>) -> Self {
        SourceEndpoint::from(Iohos::from(hos))
    }
}

impl<C> fmt::Display for SourceEndpoint<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .as_ref()
            .map_io(|()| '-'.fmt(f))
            .map_host(|pb| pb.display().fmt(f))
            .map_store(|sp| sp.fmt(f))
            .transpose()
            .map(Iohos::distill)
    }
}

impl<C> fmt::Debug for SourceEndpoint<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SourceEndpoint<{self}>")
    }
}

impl<C> FromStr for SourceEndpoint<C>
where
    C: Serialize + DeserializeOwned,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Iohos::*;

        (if s == "-" {
            Ok(MkStdio(()))
        } else {
            (if s.starts_with(SCHEME_PREFIX) {
                s.parse().map(MkStore)
            } else {
                s.parse().map(MkHost).map_err(anyhow::Error::from)
            })
            .map(MkHos)
        })
        .map(SourceEndpoint)
    }
}

#[cfg(test)]
mod tests;
