use std::{fmt, path::PathBuf, str::FromStr};

use anyhow::{anyhow, Result};
use pangalactic_dag_transfer::{BranchIter, Destination, LeafDestination};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::SCHEME_PREFIX;
use pangalactic_linkpath::{LinkDestination, LinkPath};
use pangalactic_store::Store;
use serde::{de::DeserializeOwned, Serialize};
use tokio::io::AsyncRead;

use crate::SourceEndpoint;

use self::DestinationEndpoint::*;

#[derive(Clone)]
pub enum DestinationEndpoint<C> {
    Stdout,
    Host(PathBuf),
    Store(Option<LinkDestination<C>>),
}

impl<S> Destination<S> for DestinationEndpoint<S::CID>
where
    S: Store,
{
    async fn sink_branch<B>(self, store: &mut LinkDirectoryLayer<S>, branch: B) -> Result<Self::CID>
    where
        B: fmt::Debug + Send + BranchIter<S>,
    {
        match self {
            Stdout => Err(anyhow!("cannot transfer directory to stdout: {branch:?}")),
            Host(p) => p.sink_branch(store, branch).await.map(SourceEndpoint::Host),
            Store(optp) => {
                if let Some(p) = optp {
                    p.sink_branch(store, branch)
                        .await
                        .map(SourceEndpoint::Store)
                } else {
                    ().sink_branch(store, branch)
                        .await
                        .map(LinkPath::from)
                        .map(SourceEndpoint::Store)
                }
            }
        }
    }
}

impl<S> LeafDestination<S> for DestinationEndpoint<S::CID>
where
    S: Store,
{
    type CID = SourceEndpoint<S::CID>;

    async fn sink_leaf<L>(self, store: &mut LinkDirectoryLayer<S>, leaf: L) -> Result<Self::CID>
    where
        L: fmt::Debug + Send + AsyncRead,
    {
        match self {
            Stdout => tokio::io::stdout()
                .sink_leaf(store, leaf)
                .await
                .map(|()| SourceEndpoint::Stdin),
            Host(p) => p.sink_leaf(store, leaf).await.map(SourceEndpoint::Host),
            Store(optp) => {
                if let Some(p) = optp {
                    p.sink_leaf(store, leaf).await.map(SourceEndpoint::Store)
                } else {
                    ().sink_leaf(store, leaf)
                        .await
                        .map(LinkPath::from)
                        .map(SourceEndpoint::Store)
                }
            }
        }
    }
}

impl<C> fmt::Display for DestinationEndpoint<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stdout => '-'.fmt(f),
            Host(pb) => pb.display().fmt(f),
            Store(None) => SCHEME_PREFIX.fmt(f),
            Store(Some(sp)) => sp.fmt(f),
        }
    }
}

impl<C> fmt::Debug for DestinationEndpoint<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<C> FromStr for DestinationEndpoint<C>
where
    C: DeserializeOwned + Serialize,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            Ok(Stdout)
        } else if s == SCHEME_PREFIX {
            Ok(Store(None))
        } else if s.starts_with(SCHEME_PREFIX) {
            s.parse().map(Some).map(Store)
        } else {
            s.parse().map(Host).map_err(anyhow::Error::from)
        }
    }
}
