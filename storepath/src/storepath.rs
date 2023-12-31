use pangalactic_dir::Name;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind::File;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, derive_more::Deref)]
pub struct StorePath<K> {
    #[deref]
    link: Link<K>,
    /// Invariant: if self.link.kind() == File then path.is_empty
    path: Vec<Name>,
}

impl<K> StorePath<K> {
    pub fn new(link: Link<K>, path: Vec<Name>) -> anyhow::Result<Self>
    where
        K: fmt::Debug,
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
}

impl<K> fmt::Display for StorePath<K>
where
    K: Clone + serde::Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.path.is_empty() {
            self.link.fmt(f)
        } else {
            write!(f, "{}/{}", self.link, self.path.join("/"))
        }
    }
}

impl<K> FromStr for StorePath<K>
where
    K: fmt::Debug + serde::de::DeserializeOwned,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        use std::collections::VecDeque;

        let mut q: VecDeque<&str> = s.split('/').collect();
        let linktext = q
            .pop_front()
            .ok_or_else(|| anyhow::anyhow!("missing link"))?;
        let link: Link<K> = linktext.parse()?;
        let parts = q.into_iter().map(|s| s.to_string()).collect();
        Self::new(link, parts)
    }
}

#[cfg(test)]
mod tests;
