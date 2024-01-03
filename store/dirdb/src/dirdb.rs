use crate::Writer;
use async_trait::async_trait;
use pangalactic_hash::Hash;
use pangalactic_store::Store;
use std::path::PathBuf;

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
    const SCHEME: &'static str = "pgd";

    type CID = Hash;
    type Reader = tokio::fs::File;
    type Writer = Writer;

    async fn open_reader(&mut self, key: &Self::CID) -> anyhow::Result<Self::Reader> {
        let path = self.0.join(key.to_string());
        let f = tokio::fs::File::open(path).await?;
        Ok(f)
    }

    async fn open_writer(&mut self) -> anyhow::Result<Self::Writer> {
        Writer::init(&self.0).await
    }

    async fn commit_writer(&mut self, w: Self::Writer) -> anyhow::Result<Self::CID> {
        w.commit().await
    }
}
