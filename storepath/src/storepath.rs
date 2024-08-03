use pangalactic_cid::ContentIdentifier;
use pangalactic_dir::Name;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind::File;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, derive_more::Deref)]
pub struct StorePath<C> {
    #[deref]
    link: Link<C>,
    /// Invariant: if self.link.kind() == File then path.is_empty
    path: Vec<Name>,
}

impl<C> ContentIdentifier for StorePath<C> {}

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

#[cfg(test)]
mod tests;
