use crate::{MemCid, Reader};
use async_trait::async_trait;
use pangalactic_hash::Hash;
use pangalactic_store::Store;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct MemStore(HashMap<Hash, Arc<Vec<u8>>>);

#[async_trait]
impl Store for MemStore {
    type Cid = MemCid;
    type Reader = Reader;
    type Writer = Vec<u8>;

    async fn open_reader(&mut self, key: &MemCid) -> anyhow::Result<Self::Reader> {
        self.0
            .get(&key.0)
            .cloned()
            .map(Reader::new)
            .ok_or_else(|| anyhow::Error::msg(format!("missing entry {:?}", &key)))
    }

    async fn open_writer(&mut self) -> anyhow::Result<Self::Writer> {
        Ok(Vec::new())
    }

    async fn commit_writer(&mut self, w: Self::Writer) -> anyhow::Result<Self::Cid> {
        let key = Hash::of(&w);
        self.0.insert(key.clone(), Arc::new(w));
        Ok(MemCid(key))
    }
}
