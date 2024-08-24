use not_empty::{NonEmptySlice, NonEmptyVec};
use pangalactic_bindref::Bindable;
use pangalactic_link::Link;
use pangalactic_name::Name;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, fmt::Display, str::FromStr};

use crate::LinkPath;

#[derive(Clone, derive_more::Deref)]
pub struct LinkDestination<C> {
    /// Invariant: self.link.kind() == Dir
    #[deref]
    link: Link<C>,
    path: NonEmptyVec<Name>,
}

impl<C> Bindable for LinkDestination<C> {}

impl<C> LinkDestination<C> {
    pub fn new<P>(link: Link<C>, path: P) -> anyhow::Result<Self>
    where
        NonEmptyVec<Name>: TryFrom<P>,
        <NonEmptyVec<Name> as TryFrom<P>>::Error: std::error::Error + Send + Sync + 'static,
    {
        use pangalactic_linkkind::LinkKind::Dir;

        link.peek_cid_kind(Dir)?;
        let path = NonEmptyVec::try_from(path)?;
        Ok(LinkDestination { link, path })
    }

    pub fn link(&self) -> &Link<C> {
        &self.link
    }

    pub fn path(&self) -> &NonEmptySlice<Name> {
        self.path.as_slice()
    }

    pub(crate) fn replace_link_into_path(self, newroot: Link<C>) -> anyhow::Result<LinkPath<C>>
    where
        C: Serialize,
    {
        LinkPath::new(newroot, self.path.into())
    }
}

impl<C> Display for LinkDestination<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.link, self.path.join("/"))
    }
}

impl<C> Debug for LinkDestination<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl<C> FromStr for LinkDestination<C>
where
    C: DeserializeOwned,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (link, parts) = crate::parser::parse_parts(s)?;
        Self::new(link, parts)
    }
}
