mod inner;

use std::{fmt, str::FromStr};

use anyhow::Result;
use pangalactic_dag_transfer::{BranchIter, Destination, LeafDestination};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::Link;
use pangalactic_name::NonEmptyPath;
use pangalactic_store::Store;
use serde::{de::DeserializeOwned, Serialize};

use crate::LinkPath;

use self::inner::Inner;

#[derive(Clone)]
pub struct LinkDestination<C>(Option<Inner<C>>);

impl<C> LinkDestination<C> {
    pub fn new_bare() -> Self {
        LinkDestination(None)
    }

    pub fn new_linked_path<P>(link: Link<C>, path: P) -> Result<Self>
    where
        NonEmptyPath: TryFrom<P>,
        <NonEmptyPath as TryFrom<P>>::Error: std::error::Error + Send + Sync + 'static,
    {
        let inner = Inner::new(link, path)?;
        Ok(LinkDestination(Some(inner)))
    }
}

impl<S> Destination<S> for LinkDestination<S::CID>
where
    S: Store,
{
    async fn sink_branch<B>(self, store: &mut LinkDirectoryLayer<S>, branch: B) -> Result<Self::CID>
    where
        B: std::fmt::Debug + Send + BranchIter<S>,
    {
        if let Some(inner) = self.0 {
            inner.sink_branch(store, branch).await
        } else {
            ().sink_branch(store, branch).await.map(LinkPath::from)
        }
    }
}

impl<S> LeafDestination<S> for LinkDestination<S::CID>
where
    S: Store,
{
    type CID = LinkPath<S::CID>;

    async fn sink_leaf<L>(self, store: &mut LinkDirectoryLayer<S>, leaf: L) -> Result<Self::CID>
    where
        L: std::fmt::Debug + Send + tokio::io::AsyncRead,
    {
        if let Some(inner) = self.0 {
            inner.sink_leaf(store, leaf).await
        } else {
            ().sink_leaf(store, leaf).await.map(LinkPath::from)
        }
    }
}

impl<C> fmt::Display for LinkDestination<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .as_ref()
            .map(|inner| inner.fmt(f))
            .unwrap_or_else(|| pangalactic_link::SCHEME_PREFIX.fmt(f))
    }
}

impl<C> fmt::Debug for LinkDestination<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LinkDestination<{self}>")
    }
}

impl<C> FromStr for LinkDestination<C>
where
    C: DeserializeOwned + Serialize,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == pangalactic_link::SCHEME_PREFIX {
            Ok(LinkDestination(None))
        } else {
            let inner = s.parse()?;
            Ok(LinkDestination(Some(inner)))
        }
    }
}
