use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use pangalactic_hash::Hash;
use pangalactic_hash::Writer as HashWriter;
use pangalactic_store::{Commit, Load, Store};

use crate::Reader;

#[derive(Debug, Default)]
pub struct MemStore(HashMap<Hash, Arc<Vec<u8>>>);

#[async_trait]
impl Store for MemStore {
    type CID = Hash;
    type Reader = Reader;
    type Writer = HashWriter<Vec<u8>>;

    async fn open_writer(&self) -> anyhow::Result<Self::Writer> {
        Ok(HashWriter::from(Vec::new()))
    }
}

#[async_trait]
impl Load<MemStore> for Reader {
    async fn load_from_store(store: &MemStore, cid: &Hash) -> anyhow::Result<Self> {
        store
            .0
            .get(cid)
            .cloned()
            .map(Reader::new)
            .ok_or_else(|| anyhow::Error::msg(format!("missing entry {:?}", &cid)))
    }
}

#[async_trait]
impl Commit<MemStore> for HashWriter<Vec<u8>> {
    async fn commit_into_store(self, store: &mut MemStore) -> anyhow::Result<Hash> {
        let (vec, hash) = self.unwrap();
        store.0.insert(hash.clone(), Arc::new(vec));
        Ok(hash)
    }
}
