use crate::{HostDirectorySerializationContainer, Name};
use derive_more::{Deref, DerefMut, From, Into};
use pangalactic_dir::Directory;
use pangalactic_link::Link;
use pangalactic_store::Store;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deref, DerefMut, From, Into, Deserialize, Serialize)]
#[serde(
    bound = "S: Store",
    try_from = "HostDirectorySerializationContainer<S>",
    into = "HostDirectorySerializationContainer<S>"
)]
pub struct HostDirectory<S>(pub(crate) Inner<S>)
where
    S: Store;

pub(crate) type Inner<S> = Directory<Link<S>>;

impl<S> Clone for HostDirectory<S>
where
    S: Store,
{
    fn clone(&self) -> Self {
        HostDirectory(self.0.clone())
    }
}

impl<S> Default for HostDirectory<S>
where
    S: Store,
{
    fn default() -> Self {
        HostDirectory(Directory::default())
    }
}

impl<S> PartialEq for HostDirectory<S>
where
    S: Store,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<N, S> FromIterator<(N, Link<S>)> for HostDirectory<S>
where
    Name: From<N>,
    S: Store,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (N, Link<S>)>,
    {
        HostDirectory(Directory::from_iter(iter))
    }
}

impl<S> IntoIterator for HostDirectory<S>
where
    S: Store,
{
    type Item = (Name, Link<S>);
    type IntoIter = <Directory<Link<S>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
