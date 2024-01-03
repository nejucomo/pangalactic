use crate::{Name, NameRef};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Directory<L>(pub(crate) BTreeMap<Name, L>);

impl<L> Default for Directory<L> {
    fn default() -> Self {
        Directory(BTreeMap::default())
    }
}

impl<N, L> FromIterator<(N, L)> for Directory<L>
where
    Name: From<N>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (N, L)>,
    {
        Directory(BTreeMap::from_iter(
            iter.into_iter().map(|(n, link)| (Name::from(n), link)),
        ))
    }
}

impl<L> IntoIterator for Directory<L> {
    type Item = (Name, L);
    type IntoIter = <BTreeMap<Name, L> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<L> Directory<L> {
    pub fn insert(&mut self, name: Name, link: L) -> anyhow::Result<()> {
        let errname = name.clone();
        if self.0.insert(name, link).is_none() {
            Ok(())
        } else {
            Err(anyhow::Error::msg(format!(
                "duplicate entry for {errname:?}"
            )))
        }
    }

    pub fn overwrite(&mut self, name: Name, link: L) {
        self.0.insert(name, link);
    }

    pub fn get(&self, name: &NameRef) -> Option<&L> {
        self.0.get(name)
    }

    pub fn remove(&mut self, name: &NameRef) -> Option<L> {
        self.0.remove(name)
    }

    pub fn remove_required(&mut self, name: &NameRef) -> anyhow::Result<L> {
        self.remove(name)
            .ok_or_else(|| anyhow::Error::msg(format!("missing required name {name:?}")))
    }

    pub fn require_empty(self) -> anyhow::Result<()> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(anyhow::Error::msg(format!(
                "unexpected entries {:?}",
                self.0.into_keys().collect::<Vec<Name>>()
            )))
        }
    }
}
