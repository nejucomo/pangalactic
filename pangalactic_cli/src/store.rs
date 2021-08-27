use pangalactic_cryptostore::{CryptoStore, ReadCap};
use pangalactic_dirstore::{DirStore, Key as DirKey};
use pangalactic_fs::ensure_directory_exists;
use pangalactic_node::Link;
use pangalactic_nodestore::NodeStore;
use std::ops::{Deref, DerefMut};
use std::path::Path;

type NodeStoreImpl = NodeStore<CryptoStore<DirStore>>;

#[derive(Debug)]
pub struct PgStore(NodeStoreImpl);

pub type PgLink = Link<ReadCap<DirKey>>;

impl PgStore {
    pub fn open<P: AsRef<Path>>(datadir: P) -> std::io::Result<PgStore> {
        let storedir = datadir.as_ref().join("store");
        ensure_directory_exists(&storedir)?;
        Ok(PgStore(NodeStore::from(CryptoStore::from(DirStore::from(
            storedir,
        )))))
    }
}

impl Deref for PgStore {
    type Target = NodeStoreImpl;

    fn deref(&self) -> &NodeStoreImpl {
        &self.0
    }
}

impl DerefMut for PgStore {
    fn deref_mut(&mut self) -> &mut NodeStoreImpl {
        &mut self.0
    }
}
