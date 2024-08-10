use std::fmt;
use std::str::FromStr;

use async_trait::async_trait;
use pangalactic_cid::ContentIdentifier;
use pangalactic_dir::Name;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind::File;
use pangalactic_store::{Commit, Store};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::PathLayer;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        C: fmt::Debug,
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

    pub fn unwrap_pathless_link(self) -> anyhow::Result<Link<C>>
    where
        C: fmt::Debug,
    {
        if self.path.is_empty() {
            Ok(self.link)
        } else {
            Err(anyhow::anyhow!("expected bare link with no path segments")
                .context(format!("{self:?}")))
        }
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

impl<C> FromStr for StorePath<C>
where
    C: fmt::Debug + DeserializeOwned,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (link, parts) = crate::parser::parse_parts(s)?;
        Self::new(link, parts)
    }
}

#[async_trait]
impl<S> Commit<PathLayer<S>> for StorePath<S::CID>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut PathLayer<S>,
    ) -> anyhow::Result<StorePath<S::CID>> {
        store.commit(&self).await
    }
}

#[async_trait]
impl<'a, S> Commit<PathLayer<S>> for &'a StorePath<S::CID>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut PathLayer<S>,
    ) -> anyhow::Result<StorePath<S::CID>> {
        store.resolve_path(self).await.map(StorePath::from)
    }
}

#[cfg(test)]
mod tests;
