use std::error::Error as StdError;
use std::fmt;
use std::str::FromStr;

use anyhow::Result;
use pangalactic_cid::ContentIdentifier;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind::File;
use pangalactic_name::{Path, PathRef};
use pangalactic_store::{Commit, Store};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::PathLayerExt;

// TODO: Switch to enum with { DirLP(CID, Path) ; FileLP(CID) }
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LinkPath<C> {
    link: Link<C>,
    /// Invariant: if self.link.kind() == File then path.is_empty()
    path: Path,
}

impl<C> ContentIdentifier for LinkPath<C> where
    C: Clone + fmt::Debug + Eq + PartialEq + DeserializeOwned + Serialize + Send + Sync
{
}

impl<C> LinkPath<C> {
    pub fn new<P>(link: Link<C>, path: P) -> Result<Self>
    where
        C: Serialize,
        Path: TryFrom<P>,
        <Path as TryFrom<P>>::Error: StdError + Send + Sync + 'static,
    {
        let path = Path::try_from(path)?;

        if link.kind() == File && !path.is_empty() {
            anyhow::bail!("file link {:?} cannot have path path {:?}", link, path);
        }

        Ok(LinkPath { link, path })
    }

    pub fn link(&self) -> &Link<C> {
        &self.link
    }

    pub fn path(&self) -> &PathRef {
        self.path.as_ref()
    }
}

impl<C> From<(Link<C>, Path)> for LinkPath<C> {
    fn from((link, path): (Link<C>, Path)) -> Self {
        LinkPath { link, path }
    }
}

impl<C> From<LinkPath<C>> for (Link<C>, Path) {
    fn from(lp: LinkPath<C>) -> Self {
        (lp.link, lp.path)
    }
}

impl<C> From<Link<C>> for LinkPath<C> {
    fn from(link: Link<C>) -> Self {
        LinkPath {
            link,
            path: Path::default(),
        }
    }
}

impl<C> fmt::Display for LinkPath<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.path.is_empty() {
            self.link.fmt(f)
        } else {
            write!(f, "{}/{}", self.link, self.path)
        }
    }
}

impl<C> fmt::Debug for LinkPath<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<C> FromStr for LinkPath<C>
where
    C: Serialize + DeserializeOwned,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (linktext, path) = if let Some((linktext, pathtext)) = s.split_once('/') {
            let path = Path::try_from(pathtext)?;
            (linktext, path)
        } else {
            (s, Path::default())
        };
        let link = linktext.parse()?;
        Self::new(link, path)
    }
}

impl<S> Commit<LinkDirectoryLayer<S>> for LinkPath<S::CID>
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>> {
        store.commit(&self).await
    }
}

impl<'a, S> Commit<LinkDirectoryLayer<S>> for &'a LinkPath<S::CID>
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>> {
        store.resolve_path(self).await
    }
}

#[cfg(test)]
mod tests;
