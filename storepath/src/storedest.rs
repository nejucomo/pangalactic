use crate::StorePath::{self, DirPath};
use not_empty::{NonEmptySlice, NonEmptyVec};
use pangalactic_dir::Name;
use pangalactic_link::Link;
use pangalactic_store::StoreCid;
use std::fmt;
use std::str::FromStr;

// TODO: Replace `intermediates/lastname` with NonEmptyVec
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreDestination<K>
where
    K: StoreCid,
{
    key: K,
    relpath: NonEmptyVec<Name>,
}

impl<K> StoreDestination<K>
where
    K: StoreCid,
{
    pub fn link_and_relpath(&self) -> (Link<K>, &NonEmptySlice<Name>) {
        use pangalactic_linkkind::LinkKind::Dir;

        (Link::new(Dir, self.key.clone()), self.relpath.as_slice())
    }

    pub fn prefix_path(&self, components: usize) -> StorePath<K> {
        {
            // BUG: assertion of undocumented precondition:
            let relpath = &self.relpath;
            assert!(
                components <= usize::from(relpath.len()),
                "{components:?} vs {relpath:?}"
            );
        }
        StorePath::DirPath(
            self.key.clone(),
            self.relpath[..components]
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>(),
        )
    }
}

impl<K> From<StoreDestination<K>> for StorePath<K>
where
    K: StoreCid,
{
    fn from(dest: StoreDestination<K>) -> Self {
        let StoreDestination { key, relpath } = dest;
        DirPath(key, Vec::from(relpath))
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
            DirPath(key, relpath) => {
                let relpath = NonEmptyVec::try_from(relpath)?;
                Ok(StoreDestination { key, relpath })
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
