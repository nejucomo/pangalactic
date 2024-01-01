use not_empty::NonEmptyVec;
use pangalactic_dir::Name;
use pangalactic_link::Link;
use std::{fmt::Debug, fmt::Display, str::FromStr};

#[derive(Debug, Clone, derive_more::Deref)]
pub struct StoreDestination<K> {
    /// Invariant: self.link.kind() == Dir
    #[deref]
    link: Link<K>,
    path: NonEmptyVec<Name>,
}

impl<K> StoreDestination<K> {
    pub fn new<P>(link: Link<K>, path: P) -> anyhow::Result<Self>
    where
        K: Debug,
        NonEmptyVec<Name>: TryFrom<P>,
        <NonEmptyVec<Name> as TryFrom<P>>::Error: std::error::Error + Send + Sync + 'static,
    {
        use pangalactic_linkkind::LinkKind::Dir;

        link.peek_key_kind(Dir)?;
        let path = NonEmptyVec::try_from(path)?;
        Ok(StoreDestination { link, path })
    }
}

impl<K> Display for StoreDestination<K>
where
    K: Clone + serde::Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.link, self.path.join("/"))
    }
}

impl<K> FromStr for StoreDestination<K>
where
    K: Debug + serde::de::DeserializeOwned,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (link, parts) = crate::parser::parse_parts(s)?;
        Self::new(link, parts)
    }
}
