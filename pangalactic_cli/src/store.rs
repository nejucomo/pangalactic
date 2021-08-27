use pangalactic_cryptostore::{CryptoStore, ReadCap};
use pangalactic_dirstore::{DirStore, Key as DirKey};
use pangalactic_fs::ensure_directory_exists;
use pangalactic_node::Link;
use std::path::Path;
// use pangalactic_store::Store;

#[derive(Debug)]
pub struct PgStore(CryptoStore<DirStore>);

#[derive(Debug)]
pub struct Key(Link<ReadCap<DirKey>>);

impl PgStore {
    pub fn open<P: AsRef<Path>>(datadir: P) -> std::io::Result<PgStore> {
        let storedir = datadir.as_ref().join("store");
        ensure_directory_exists(&storedir)?;
        Ok(PgStore(CryptoStore::from(DirStore::from(storedir))))
    }
}
