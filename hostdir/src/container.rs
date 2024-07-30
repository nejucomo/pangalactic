use crate::hostdir::Inner;
use crate::HostDirectory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct HostDirectorySerializationContainer<C> {
    version: u64,
    inner: Inner<C>,
}

const SERIALIZATION_VERSION: u64 = 0;

impl<C> TryFrom<HostDirectorySerializationContainer<C>> for HostDirectory<C> {
    type Error = anyhow::Error;

    fn try_from(container: HostDirectorySerializationContainer<C>) -> Result<Self, Self::Error> {
        if container.version == SERIALIZATION_VERSION {
            Ok(HostDirectory(container.inner))
        } else {
            anyhow::bail!(
                "unknown serialization version {:?}; expected {:?}",
                container.version,
                SERIALIZATION_VERSION
            );
        }
    }
}

impl<C> From<HostDirectory<C>> for HostDirectorySerializationContainer<C> {
    fn from(hd: HostDirectory<C>) -> Self {
        HostDirectorySerializationContainer {
            version: SERIALIZATION_VERSION,
            inner: hd.0,
        }
    }
}
