use crate::{HostDirectorySerializationContainer, Name};
use derive_more::{Deref, DerefMut, From, Into};
use pangalactic_dir::Directory;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, Deref, DerefMut, From, Into, Deserialize, Serialize, PartialEq)]
#[serde(
    bound = "C: Clone + DeserializeOwned + Serialize",
    try_from = "HostDirectorySerializationContainer<C>",
    into = "HostDirectorySerializationContainer<C>"
)]
pub struct HostDirectory<C>(pub(crate) Inner<C>);

pub(crate) type Inner<C> = Directory<Link<CidMeta<C>>>;

impl<C> Default for HostDirectory<C> {
    fn default() -> Self {
        HostDirectory(Directory::default())
    }
}

impl<N, C> FromIterator<(N, Link<CidMeta<C>>)> for HostDirectory<C>
where
    Name: From<N>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (N, Link<CidMeta<C>>)>,
    {
        HostDirectory(Directory::from_iter(iter))
    }
}

impl<C> IntoIterator for HostDirectory<C> {
    type Item = (Name, Link<CidMeta<C>>);
    type IntoIter = <Directory<Link<CidMeta<C>>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
