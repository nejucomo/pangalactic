use pangalactic_dir::Name;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind::{Dir, File};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorePath<K> {
    FilePath(K),
    DirPath(K, Vec<Name>),
}
use StorePath::*;

impl<K> From<StorePath<K>> for (Link<K>, Vec<Name>) {
    fn from(sp: StorePath<K>) -> Self {
        match sp {
            FilePath(key) => (Link::new(File, key), vec![]),
            DirPath(key, suffix) => (Link::new(Dir, key), suffix),
        }
    }
}

impl<K> TryFrom<(Link<K>, Vec<Name>)> for StorePath<K>
where
    K: fmt::Debug,
{
    type Error = anyhow::Error;

    fn try_from((link, suffix): (Link<K>, Vec<Name>)) -> anyhow::Result<Self> {
        let (kind, key) = link.unwrap();

        match kind {
            File => {
                if suffix.is_empty() {
                    Ok(FilePath(key))
                } else {
                    anyhow::bail!(
                        "file link {:?} cannot have path suffix {:?}",
                        Link::new(kind, key),
                        suffix.join("/")
                    );
                }
            }
            Dir => Ok(DirPath(key, suffix)),
        }
    }
}

impl<K> fmt::Display for StorePath<K>
where
    K: Clone + serde::Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (link, mut parts) = self.clone().into();
        parts.insert(0, link.to_string());
        parts.join("/").fmt(f)
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

        Self::try_from((link, parts))
    }
}

#[cfg(test)]
mod tests;
