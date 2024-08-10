use std::path::PathBuf;

use async_trait::async_trait;
use pangalactic_hash::Hash;
use pangalactic_store::{Commit, Load, Store};

use crate::Writer;

#[derive(Debug, derive_more::From)]
pub struct DirDbStore(PathBuf);

impl Default for DirDbStore {
    fn default() -> Self {
        let d = crate::default_path();
        std::fs::create_dir_all(&d).unwrap();
        DirDbStore(d)
    }
}

#[async_trait]
impl Store for DirDbStore {
    type CID = Hash;
    type Reader = tokio::fs::File;
    type Writer = Writer;

    async fn open_writer(&self) -> anyhow::Result<Self::Writer> {
        Writer::init(&self.0).await
    }
}

#[async_trait]
impl Load<DirDbStore> for tokio::fs::File {
    async fn load_from_store(store: &DirDbStore, cid: &Hash) -> anyhow::Result<Self> {
        let path = store.0.join(cid.to_string());
        let f = tokio::fs::File::open(path).await?;
        Ok(f)
    }
}

#[async_trait]
impl Commit<DirDbStore> for Writer {
    async fn commit_into_store(self, _: &mut DirDbStore) -> anyhow::Result<Hash> {
        self.commit().await
    }
}
