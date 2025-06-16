use crate::linkdir::Inner;
use crate::LinkDirectory;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct LinkDirectorySerializationContainer<C> {
    version: u64,
    inner: Inner<C>,
}

const SERIALIZATION_VERSION: u64 = 0;

impl<C> TryFrom<LinkDirectorySerializationContainer<C>> for LinkDirectory<C> {
    type Error = anyhow::Error;

    fn try_from(container: LinkDirectorySerializationContainer<C>) -> Result<Self, Self::Error> {
        if container.version == SERIALIZATION_VERSION {
            Ok(LinkDirectory(container.inner))
        } else {
            anyhow::bail!(
                "unknown serialization version {:?}; expected {:?}",
                container.version,
                SERIALIZATION_VERSION
            );
        }
    }
}

impl<C> From<LinkDirectory<C>> for LinkDirectorySerializationContainer<C> {
    fn from(hd: LinkDirectory<C>) -> Self {
        LinkDirectorySerializationContainer {
            version: SERIALIZATION_VERSION,
            inner: hd.0,
        }
    }
}
