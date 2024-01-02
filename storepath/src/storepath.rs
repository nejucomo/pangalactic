use pangalactic_dir::Name;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind::File;
use pangalactic_store::Store;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, derive_more::Deref)]
pub struct StorePath<S>
where
    S: Store,
{
    #[deref]
    link: Link<S>,
    /// Invariant: if self.link.kind() == File then path.is_empty
    path: Vec<Name>,
}

impl<S> StorePath<S>
where
    S: Store,
{
    pub fn new(link: Link<S>, path: Vec<Name>) -> anyhow::Result<Self> {
        if link.kind() == File && !path.is_empty() {
            anyhow::bail!(
                "file link {:?} cannot have path path {:?}",
                link,
                path.join("/")
            );
        }

        Ok(StorePath { link, path })
    }
}

impl<S> Clone for StorePath<S>
where
    S: Store,
{
    fn clone(&self) -> Self {
        StorePath {
            link: self.link.clone(),
            path: self.path.clone(),
        }
    }
}

impl<S> fmt::Display for StorePath<S>
where
    S: Store,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.path.is_empty() {
            self.link.fmt(f)
        } else {
            write!(f, "{}/{}", self.link, self.path.join("/"))
        }
    }
}

impl<S> FromStr for StorePath<S>
where
    S: Store,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (link, parts) = crate::parser::parse_parts(s)?;
        Self::new(link, parts)
    }
}

#[cfg(test)]
mod tests;
