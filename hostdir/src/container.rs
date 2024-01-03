use crate::hostdir::Inner;
use crate::HostDirectory;
use pangalactic_store::Store;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(bound = "S: Store")]
pub(crate) struct HostDirectorySerializationContainer<S>
where
    S: Store,
{
    version: u64,
    inner: Inner<S>,
}

const SERIALIZATION_VERSION: u64 = 0;

impl<S> TryFrom<HostDirectorySerializationContainer<S>> for HostDirectory<S>
where
    S: Store,
{
    type Error = anyhow::Error;

    fn try_from(container: HostDirectorySerializationContainer<S>) -> Result<Self, Self::Error> {
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

impl<S> From<HostDirectory<S>> for HostDirectorySerializationContainer<S>
where
    S: Store,
{
    fn from(hd: HostDirectory<S>) -> Self {
        HostDirectorySerializationContainer {
            version: SERIALIZATION_VERSION,
            inner: hd.0,
        }
    }
}
