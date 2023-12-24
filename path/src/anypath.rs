use crate::StorePath;
use pangalactic_store::StoreCid;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug, derive_more::From)]
pub enum AnyPath<K>
where
    K: StoreCid,
{
    Host(PathBuf),
    Store(StorePath<K>),
}

impl<K> fmt::Display for AnyPath<K>
where
    K: StoreCid,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AnyPath::*;

        match self {
            Host(p) => p.display().fmt(f),
            Store(p) => p.fmt(f),
        }
    }
}

impl<K> FromStr for AnyPath<K>
where
    K: StoreCid,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AnyPath::*;

        if s.split_once("://").is_some() {
            let p = StorePath::from_str(s)?;
            Ok(Store(p))
        } else {
            let p = PathBuf::from_str(s)?;
            Ok(Host(p))
        }
    }
}
