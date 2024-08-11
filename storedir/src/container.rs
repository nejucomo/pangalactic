use crate::storedir::Inner;
use crate::StoreDirectory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct StoreDirectorySerializationContainer<C> {
    version: u64,
    inner: Inner<C>,
}

const SERIALIZATION_VERSION: u64 = 0;

impl<C> TryFrom<StoreDirectorySerializationContainer<C>> for StoreDirectory<C> {
    type Error = anyhow::Error;

    fn try_from(container: StoreDirectorySerializationContainer<C>) -> Result<Self, Self::Error> {
        if container.version == SERIALIZATION_VERSION {
            Ok(StoreDirectory(container.inner))
        } else {
            anyhow::bail!(
                "unknown serialization version {:?}; expected {:?}",
                container.version,
                SERIALIZATION_VERSION
            );
        }
    }
}

impl<C> From<StoreDirectory<C>> for StoreDirectorySerializationContainer<C> {
    fn from(hd: StoreDirectory<C>) -> Self {
        StoreDirectorySerializationContainer {
            version: SERIALIZATION_VERSION,
            inner: hd.0,
        }
    }
}
