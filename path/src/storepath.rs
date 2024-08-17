use std::fmt;
use std::str::FromStr;

use pangalactic_cid::ContentIdentifier;
use pangalactic_dir::Name;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind::File;
use pangalactic_store::{Commit, Store};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::PathLayerExt;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StorePath<C> {
    link: Link<C>,
    /// Invariant: if self.link.kind() == File then path.is_empty
    path: Vec<Name>,
}

impl<C> ContentIdentifier for StorePath<C> where
    C: Clone + fmt::Debug + Eq + PartialEq + DeserializeOwned + Serialize + Send + Sync
{
}

impl<C> StorePath<C> {
    pub fn new(link: Link<C>, path: Vec<Name>) -> anyhow::Result<Self>
    where
        C: Serialize,
    {
        if link.kind() == File && !path.is_empty() {
            anyhow::bail!(
                "file link {:?} cannot have path path {:?}",
                link,
                path.join("/")
            );
        }

        Ok(StorePath { link, path })
    }

    pub fn link(&self) -> &Link<C> {
        &self.link
    }

    pub fn path(&self) -> &[Name] {
        self.path.as_slice()
    }
}

impl<C> From<Link<C>> for StorePath<C> {
    fn from(link: Link<C>) -> Self {
        StorePath { link, path: vec![] }
    }
}

impl<C> fmt::Display for StorePath<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.path.is_empty() {
            self.link.fmt(f)
        } else {
            write!(f, "{}/{}", self.link, self.path.join("/"))
        }
    }
}

impl<C> fmt::Debug for StorePath<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<C> FromStr for StorePath<C>
where
    C: Serialize + DeserializeOwned,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (link, parts) = crate::parser::parse_parts(s)?;
        Self::new(link, parts)
    }
}

impl<S> Commit<LinkDirectoryLayer<S>> for StorePath<S::CID>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        store.commit(&self).await
    }
}

impl<'a, S> Commit<LinkDirectoryLayer<S>> for &'a StorePath<S::CID>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        store.resolve_path(self).await
    }
}

#[cfg(test)]
mod tests;
