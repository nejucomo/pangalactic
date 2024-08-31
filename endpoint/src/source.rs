mod branch;
mod leaf;

use std::{fmt, path::PathBuf, str::FromStr};

use anyhow::Result;
use pangalactic_dag_transfer::{IntoSource, Source};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::SCHEME_PREFIX;
use pangalactic_linkpath::LinkPath;
use pangalactic_store::Store;
use serde::{de::DeserializeOwned, Serialize};

pub use self::branch::SourceEndpointBranch;
pub use self::leaf::SourceEndpointLeaf;
use self::SourceEndpoint::*;

#[derive(Eq, PartialEq)]
pub enum SourceEndpoint<C> {
    Stdin,
    Host(PathBuf),
    Store(LinkPath<C>),
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
        use Source::*;

        match self {
            Stdin => Ok(Leaf(SourceEndpointLeaf::Stdin(tokio::io::stdin()))),
            Host(p) => match p.into_source(store).await? {
                Leaf(l) => Ok(Leaf(SourceEndpointLeaf::Host(l))),
                Branch(b) => Ok(Branch(SourceEndpointBranch::Host(b))),
            },
            Store(p) => match p.into_source(store).await? {
                Leaf(l) => Ok(Leaf(SourceEndpointLeaf::Store(l))),
                Branch(b) => Ok(Branch(SourceEndpointBranch::Store(b))),
            },
        }
    }
}

impl<C> fmt::Display for SourceEndpoint<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SourceEndpoint::*;

        match self {
            Stdin => '-'.fmt(f),
            Host(pb) => pb.display().fmt(f),
            Store(sp) => sp.fmt(f),
        }
    }
}

impl<C> fmt::Debug for SourceEndpoint<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<C> FromStr for SourceEndpoint<C>
where
    C: Serialize + DeserializeOwned,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SourceEndpoint::*;
        if s == "-" {
            Ok(Stdin)
        } else if s.starts_with(SCHEME_PREFIX) {
            s.parse().map(Store)
        } else {
            s.parse().map(Host).map_err(anyhow::Error::from)
        }
    }
}

#[cfg(test)]
mod tests;
