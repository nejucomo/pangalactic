use std::fmt;
use std::path::PathBuf;

use pangalactic_linkpath::LinkPath;
use serde::Serialize;

use crate::hos::Hos::{self, MkHost, MkStore};
use crate::iohos::Iohos::{self, MkHos, MkStdio};

/// Indicates the `CID` of a destination
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Receipt<C>(Iohos<(), PathBuf, LinkPath<C>>)
where
    C: Serialize;

impl<C> Receipt<C>
where
    C: Serialize,
{
    pub fn is_stdout(&self) -> bool {
        matches!(self.0, Iohos::MkStdio(()))
    }
}

impl<C> From<()> for Receipt<C>
where
    C: Serialize,
{
    fn from(v: ()) -> Self {
        Receipt::from(MkStdio(v))
    }
}

impl<C> From<PathBuf> for Receipt<C>
where
    C: Serialize,
{
    fn from(p: PathBuf) -> Self {
        Receipt::from(MkHost(p))
    }
}

impl<C> From<LinkPath<C>> for Receipt<C>
where
    C: Serialize,
{
    fn from(p: LinkPath<C>) -> Self {
        Receipt::from(MkStore(p))
    }
}

impl<C> From<Hos<PathBuf, LinkPath<C>>> for Receipt<C>
where
    C: Serialize,
{
    fn from(hos: Hos<PathBuf, LinkPath<C>>) -> Self {
        Receipt::from(MkHos(hos))
    }
}

impl<C> From<Iohos<(), PathBuf, LinkPath<C>>> for Receipt<C>
where
    C: Serialize,
{
    fn from(value: Iohos<(), PathBuf, LinkPath<C>>) -> Self {
        Receipt(value)
    }
}

impl<C> fmt::Display for Receipt<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
