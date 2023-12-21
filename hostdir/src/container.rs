use crate::HostDirectory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct HostDirectorySerializationContainer<K>
where
    K: Clone,
{
    version: u64,
    hd: HostDirectory<K>,
}

const SERIALIZATION_VERSION: u64 = 0;

impl<K> TryFrom<HostDirectorySerializationContainer<K>> for HostDirectory<K>
where
    K: Clone,
{
    type Error = anyhow::Error;

    fn try_from(container: HostDirectorySerializationContainer<K>) -> Result<Self, Self::Error> {
        if container.version == SERIALIZATION_VERSION {
            Ok(container.hd)
        } else {
            anyhow::bail!(
                "unknown serialization version {:?}; expected {:?}",
                container.version,
                SERIALIZATION_VERSION
            );
        }
    }
}

impl<K> From<HostDirectory<K>> for HostDirectorySerializationContainer<K>
where
    K: Clone,
{
    fn from(hd: HostDirectory<K>) -> Self {
        HostDirectorySerializationContainer {
            version: SERIALIZATION_VERSION,
            hd,
        }
    }
}
