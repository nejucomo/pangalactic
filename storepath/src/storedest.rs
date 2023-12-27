use crate::StorePath::{self, DirPath};
use pangalactic_dir::Name;
use pangalactic_store::StoreCid;
use std::fmt;
use std::str::FromStr;

// TODO: Replace `intermediate/lastname` with NonEmptyVec
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreDestination<K>
where
    K: StoreCid,
{
    key: K,
    intermediate: Vec<Name>,
    lastname: Name,
}

impl<K> From<StoreDestination<K>> for StorePath<K>
where
    K: StoreCid,
{
    fn from(dest: StoreDestination<K>) -> Self {
        let StoreDestination {
            key,
            mut intermediate,
            lastname,
        } = dest;
        intermediate.push(lastname);
        DirPath(key, intermediate)
    }
}

impl<K> TryFrom<StorePath<K>> for StoreDestination<K>
where
    K: StoreCid,
{
    type Error = anyhow::Error;

    fn try_from(sp: StorePath<K>) -> Result<Self, Self::Error> {
        use anyhow::bail;

        match sp {
            DirPath(key, mut intermediate) => {
                if let Some(lastname) = intermediate.pop() {
                    Ok(StoreDestination {
                        key,
                        intermediate,
                        lastname,
                    })
                } else {
                    bail!("a dir StorePath must have at least one path name compone tto be a StoreDestination")
                }
            }
            _ => bail!("a file StorePath cannot be a StoreDestination"),
        }
    }
}

impl<K> fmt::Display for StoreDestination<K>
where
    K: StoreCid,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Inefficient runtime, low effort code:
        StorePath::from(self.clone()).fmt(f)
    }
}

impl<K> FromStr for StoreDestination<K>
where
    K: StoreCid,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sp: StorePath<K> = s.parse()?;
        Self::try_from(sp)
    }
}
