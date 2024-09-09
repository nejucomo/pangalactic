mod receipt;

use std::{fmt, future::ready, path::PathBuf, str::FromStr};

use anyhow::{anyhow, Result};
use pangalactic_dag_transfer::{BranchIter, Destination, LeafDestination};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::SCHEME_PREFIX;
use pangalactic_linkpath::LinkDestination;
use pangalactic_store::Store;
use serde::{de::DeserializeOwned, Serialize};
use tokio::io::AsyncRead;

use crate::iohos::Iohos;

pub use self::receipt::Receipt;

#[derive(Clone)]
pub struct DestinationEndpoint<C>(Inner<C>);

type Inner<C> = Iohos<(), PathBuf, LinkDestination<C>>;

impl<C> DestinationEndpoint<C> {
    pub const fn for_stdout() -> Self {
        DestinationEndpoint(Iohos::MkStdio(()))
    }
}

impl<S> Destination<S> for DestinationEndpoint<S::CID>
where
    S: Store,
{
    async fn sink_branch<B>(self, store: &mut LinkDirectoryLayer<S>, branch: B) -> Result<Self::CID>
    where
        B: fmt::Debug + Send + BranchIter<S>,
    {
        self.0
            .map_any_with(
                (store, branch),
                |_io, branch| {
                    ready(anyhow::Result::<Self::CID>::Err(anyhow!(
                        "cannot transfer directory to stdout: {branch:?}"
                    )))
                },
                |h, (store, branch)| h.sink_branch(store, branch),
                |s, (store, branch)| s.sink_branch(store, branch),
            )
            .await_futures()
            .await
            .transpose()
            .map(|iohos| iohos.project_into(Receipt::from, Receipt::from, Receipt::from))
    }
}

impl<S> LeafDestination<S> for DestinationEndpoint<S::CID>
where
    S: Store,
{
    type CID = Receipt<S::CID>;

    async fn sink_leaf<L>(self, store: &mut LinkDirectoryLayer<S>, leaf: L) -> Result<Self::CID>
    where
        L: fmt::Debug + Send + AsyncRead,
    {
        self.0
            .map_any_with(
                (store, leaf),
                |(), (store, leaf)| tokio::io::stdout().sink_leaf(store, leaf),
                |p, (store, leaf)| p.sink_leaf(store, leaf),
                |p, (store, leaf)| p.sink_leaf(store, leaf),
            )
            .await_futures()
            .await
            .transpose()
            .map(Receipt::from)
    }
}

impl<C> fmt::Display for DestinationEndpoint<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.as_ref().project_into_with(
            f,
            |(), f| '-'.fmt(f),
            |pathbuf, f| pathbuf.display().fmt(f),
            |linkdest, f| linkdest.fmt(f),
        )
    }
}

impl<C> fmt::Debug for DestinationEndpoint<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DestinationEndpoint<{self}>")
    }
}

impl<C> FromStr for DestinationEndpoint<C>
where
    C: DeserializeOwned + Serialize,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        parse_inner(s).map(DestinationEndpoint)
    }
}

fn parse_inner<C>(s: &str) -> Result<Inner<C>>
where
    C: Serialize + DeserializeOwned,
{
    use crate::hos::Hos::*;
    use Iohos::*;

    if s == "-" {
        Ok(MkStdio(()))
    } else if s.starts_with(SCHEME_PREFIX) {
        s.parse().map(MkStore).map(MkHos)
    } else {
        s.parse()
            .map(MkHost)
            .map_err(anyhow::Error::from)
            .map(MkHos)
    }
}
