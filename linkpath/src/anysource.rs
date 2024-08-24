use std::{fmt, path::PathBuf, str::FromStr};

use pangalactic_iowrappers::Readable;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::{Link, SCHEME_PREFIX};
use pangalactic_store::{Commit, Store};
use serde::{de::DeserializeOwned, Serialize};

use crate::StorePath;

#[derive(Clone, Eq, PartialEq)]
pub enum AnySource<C> {
    Stdin,
    Host(PathBuf),
    Store(StorePath<C>),
}
use AnySource::*;

impl<C> fmt::Display for AnySource<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stdin => '-'.fmt(f),
            Host(pb) => pb.display().fmt(f),
            Store(sp) => sp.fmt(f),
        }
    }
}

impl<C> fmt::Debug for AnySource<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<C> FromStr for AnySource<C>
where
    C: Serialize + DeserializeOwned,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            Ok(Stdin)
        } else if s.starts_with(SCHEME_PREFIX) {
            s.parse().map(Store)
        } else {
            s.parse().map(Host).map_err(anyhow::Error::from)
        }
    }
}

impl<S> Commit<LinkDirectoryLayer<S>> for AnySource<S::CID>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        match self {
            Stdin => store.commit(Readable(tokio::io::stdin())).await,
            Host(p) => store.commit(p).await,
            AnySource::Store(sp) => store.commit(sp).await,
        }
    }
}

#[cfg(test)]
mod tests;
