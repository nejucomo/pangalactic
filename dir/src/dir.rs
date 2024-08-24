use std::collections::BTreeMap;
use std::error::Error as StdError;

use anyhow::{anyhow, Result};
use pangalactic_name::{Name, NameRef};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
    pub fn insert<N>(&mut self, name: N, link: L) -> Result<()>
    where
        Name: TryFrom<N>,
        <Name as TryFrom<N>>::Error: StdError + Send + Sync + 'static,
    {
        let name = Name::try_from(name)?;
        if self.0.insert(name.clone(), link).is_none() {
            Ok(())
        } else {
            Err(anyhow!("duplicate entry for {name:?}"))
        }
    }

    pub fn overwrite<N>(&mut self, name: N, link: L) -> Result<()>
    where
        Name: TryFrom<N>,
        <Name as TryFrom<N>>::Error: StdError + Send + Sync + 'static,
    {
        let name = Name::try_from(name)?;
        self.0.insert(name, link);
        Ok(())
    }

    pub fn get(&self, name: &NameRef) -> Option<&L> {
        self.0.get(name)
    }

    pub fn get_required<'a, N>(&self, name: &'a N) -> Result<&L>
    where
        N: ?Sized,
        &'a NameRef: TryFrom<&'a N>,
        <&'a NameRef as TryFrom<&'a N>>::Error: StdError + Send + Sync + 'static,
    {
        let nref = name.try_into()?;
        require(nref, self.get(nref))
    }

    pub fn remove(&mut self, name: &NameRef) -> Option<L> {
        self.0.remove(name)
    }

    pub fn remove_required<'a, N>(&mut self, name: &'a N) -> Result<L>
    where
        N: ?Sized,
        &'a NameRef: TryFrom<&'a N>,
        <&'a NameRef as TryFrom<&'a N>>::Error: StdError + Send + Sync + 'static,
    {
        let nref = name.try_into()?;
        require(nref, self.remove(nref))
    }

    pub fn require_empty(self) -> Result<()> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(anyhow!(
                "unexpected entries {:?}",
                self.0.into_keys().collect::<Vec<Name>>()
            ))
        }
    }

    pub fn map_values<F, M>(self, f: F) -> Directory<M>
    where
        F: Fn(L) -> M,
    {
        self.into_iter().map(|(n, l)| (n, f(l))).collect()
    }
}

fn require<T>(name: &NameRef, opt: Option<T>) -> Result<T> {
    opt.ok_or_else(|| anyhow!("missing required name {name:?}"))
}
