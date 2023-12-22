use crate::{HostDirectorySerializationContainer, Name};
use derive_more::{Deref, DerefMut, From, Into};
use pangalactic_dir::Directory;
use pangalactic_link::Link;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deref, DerefMut, From, Into, Deserialize, Serialize)]
#[serde(
    try_from = "HostDirectorySerializationContainer<K>",
    into = "HostDirectorySerializationContainer<K>"
)]
pub struct HostDirectory<K>(pub(crate) Inner<K>)
where
    K: Clone;

pub(crate) type Inner<K> = Directory<Link<K>>;

impl<K> Default for HostDirectory<K>
where
    K: Clone,
{
    fn default() -> Self {
        HostDirectory(Directory::default())
    }
}

impl<N, K> FromIterator<(N, Link<K>)> for HostDirectory<K>
where
    Name: From<N>,
    K: Clone,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (N, Link<K>)>,
    {
        HostDirectory(Directory::from_iter(iter))
    }
}

impl<K> IntoIterator for HostDirectory<K>
where
    K: Clone,
{
    type Item = (Name, Link<K>);
    type IntoIter = <Directory<Link<K>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
