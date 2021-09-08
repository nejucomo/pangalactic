use pangalactic_cryptostore::CryptoStore;
use pangalactic_dirstore::DirStore;
use pangalactic_fs::ensure_directory_exists;
use pangalactic_nodestore::{self as nodestore, NodeStore};
use std::ops::{Deref, DerefMut};
use std::path::Path;

#[derive(Debug)]
pub struct Storage(NodeStoreImpl);

pub type ReadEntry = nodestore::ReadEntry<NodeStoreInner>;

type NodeStoreImpl = NodeStore<NodeStoreInner>;
type NodeStoreInner = CryptoStore<DirStore>;

impl Storage {
    pub fn open_default() -> std::io::Result<Storage> {
        let appdirs = pangalactic_appdirs::appdirs_init!()?;
        Storage::open(appdirs.data)
    }

    pub fn open<P: AsRef<Path>>(datadir: P) -> std::io::Result<Storage> {
        let storedir = datadir.as_ref().to_path_buf();
        ensure_directory_exists(&storedir)?;
        Ok(Storage(NodeStore::from(CryptoStore::from(DirStore::from(
            storedir,
        )))))
    }
}

impl Deref for Storage {
    type Target = NodeStoreImpl;

    fn deref(&self) -> &NodeStoreImpl {
        &self.0
    }
}

impl DerefMut for Storage {
    fn deref_mut(&mut self) -> &mut NodeStoreImpl {
        &mut self.0
    }
}
