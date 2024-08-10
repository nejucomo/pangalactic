use pangalactic_iowrappers::Readable;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind;
use pangalactic_store::{Load, Store};

use crate::{HostDirectory, HostDirectoryLayer};

#[derive(Debug)]
pub enum DirNodeReader<S>
where
    S: Store,
{
    File(Readable<S::Reader>),
    Dir(HostDirectory<S::CID>),
}

impl<S> Load<HostDirectoryLayer<S>> for DirNodeReader<S>
where
    S: Store,
{
    async fn load_from_store(
        store: &HostDirectoryLayer<S>,
        cid: &Link<S::CID>,
    ) -> anyhow::Result<Self> {
        use DirNodeReader::*;

        let (kind, reader) = store.open_any_reader(cid).await?;
        match kind {
            LinkKind::File => Ok(File(reader)),
            LinkKind::Dir => HostDirectory::deserialize_from(reader).await.map(Dir),
        }
    }
}
