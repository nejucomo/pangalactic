use crate::StorePath::{self, DirPath};
use pangalactic_dir::{Name, NameRef};
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
    intermediates: Vec<Name>,
    lastname: Name,
}

impl<K> StoreDestination<K>
where
    K: StoreCid,
{
    pub fn link_intermediates_and_last_name(&self) -> (Link<K>, &[Name], &NameRef) {
        use pangalactic_linkkind::LinkKind::Dir;

        (
            Link::new(Dir, self.key.clone()),
            self.intermediates.as_slice(),
            &self.lastname,
        )
    }

    pub fn prefix_path(&self, components: usize) -> Self {
        {
            // BUG: assertion of undocumented precondition:
            let intermediates = &self.intermediates;
            assert!(
                components <= self.intermediates.len(),
                "{components:?} vs {intermediates:?}"
            );
        }
        StoreDestination {
            key: self.key.clone(),
            intermediates: self.intermediates[..components]
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>(),
            lastname: self.intermediates[components].to_string(),
        }
    }
}

impl<K> From<StoreDestination<K>> for StorePath<K>
where
    K: StoreCid,
{
    fn from(dest: StoreDestination<K>) -> Self {
        let StoreDestination {
            key,
            mut intermediates,
            lastname,
        } = dest;
        intermediates.push(lastname);
        DirPath(key, intermediates)
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
            DirPath(key, mut intermediates) => {
                if let Some(lastname) = intermediates.pop() {
                    Ok(StoreDestination {
                        key,
                        intermediates,
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
