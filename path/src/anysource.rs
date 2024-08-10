use std::{fmt::Display, path::PathBuf, str::FromStr};

use async_trait::async_trait;
use pangalactic_iowrappers::Readable;
use pangalactic_link::SCHEME_PREFIX;
use pangalactic_store::{Commit, Store};
use serde::{de::DeserializeOwned, Serialize};

use crate::{PathLayer, StorePath};

#[derive(Debug, Clone)]
pub enum AnySource<C> {
    Stdin,
    Host(PathBuf),
    Store(StorePath<C>),
}
use AnySource::*;

impl<C> Display for AnySource<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stdin => '-'.fmt(f),
            Host(pb) => pb.display().fmt(f),
            Store(sp) => sp.fmt(f),
        }
    }
}

impl<C> FromStr for AnySource<C>
where
    C: std::fmt::Debug + DeserializeOwned,
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

#[async_trait]
impl<S> Commit<PathLayer<S>> for AnySource<S::CID>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut PathLayer<S>,
    ) -> anyhow::Result<StorePath<S::CID>> {
        match self {
            Stdin => store.commit(Readable(tokio::io::stdin())).await,
            Host(p) => store.commit(p).await,
            AnySource::Store(sp) => store.commit(sp).await,
        }
    }
}
