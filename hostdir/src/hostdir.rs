use crate::HostDirectorySerializationContainer;
use derive_more::{Deref, DerefMut};
use pangalactic_dir::Directory;
use pangalactic_link::Link;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Deref, DerefMut, Deserialize, Serialize)]
#[serde(
    try_from = "HostDirectorySerializationContainer<K>",
    into = "HostDirectorySerializationContainer<K>"
)]
pub struct HostDirectory<K>(Directory<Link<K>>)
where
    K: Clone;
