use not_empty::{NonEmptySlice, NonEmptyVec};
use pangalactic_dir::Name;
use pangalactic_link::Link;
use pangalactic_store::Store;
use std::{fmt::Debug, fmt::Display, str::FromStr};

#[derive(Debug, derive_more::Deref)]
pub struct StoreDestination<S>
where
    S: Store,
{
    /// Invariant: self.link.kind() == Dir
    #[deref]
    link: Link<S>,
    path: NonEmptyVec<Name>,
}

impl<S> StoreDestination<S>
where
    S: Store,
{
    pub fn link(&self) -> &Link<S> {
        &self.link
    }

    pub fn path(&self) -> &NonEmptySlice<Name> {
        self.path.as_slice()
    }

    pub fn new<P>(link: Link<S>, path: P) -> anyhow::Result<Self>
    where
        NonEmptyVec<Name>: TryFrom<P>,
        <NonEmptyVec<Name> as TryFrom<P>>::Error: std::error::Error + Send + Sync + 'static,
    {
        use pangalactic_linkkind::LinkKind::Dir;

        link.peek_key_kind(Dir)?;
        let path = NonEmptyVec::try_from(path)?;
        Ok(StoreDestination { link, path })
    }
}

impl<S> Clone for StoreDestination<S>
where
    S: Store,
{
    fn clone(&self) -> Self {
        StoreDestination {
            link: self.link.clone(),
            path: self.path.clone(),
        }
    }
}

impl<S> Display for StoreDestination<S>
where
    S: Store,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.link, self.path.join("/"))
    }
}

impl<S> FromStr for StoreDestination<S>
where
    S: Store,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (link, parts) = crate::parser::parse_parts(s)?;
        Self::new(link, parts)
    }
}
