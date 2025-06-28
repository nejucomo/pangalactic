use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use pangalactic_config::datapath;
use pangalactic_hash::Hash;
use pangalactic_store::{Commit, Load, Store};

use crate::Writer;

#[derive(Clone, Debug, derive_more::From)]
pub struct DirDbStore(PathBuf);

impl Default for DirDbStore {
    fn default() -> Self {
        let d = datapath::get("dirdb");
        DirDbStore(d)
    }
}

impl Store for DirDbStore {
    type CID = Hash;
    type Reader = tokio::fs::File;
    type Writer = Writer;

    async fn open_writer(&self) -> anyhow::Result<Self::Writer> {
        Writer::init(&self.0).await
    }
}

impl Load<DirDbStore> for tokio::fs::File {
    async fn load_from_store(store: &DirDbStore, cid: &Hash) -> anyhow::Result<Self> {
        let path = store.0.join(cid.to_string());
        let f = tokio::fs::File::open(path).await?;
        Ok(f)
    }
}

impl Commit<DirDbStore> for Writer {
    async fn commit_into_store(self, _: &mut DirDbStore) -> anyhow::Result<Hash> {
        self.commit().await
    }
}

impl fmt::Display for DirDbStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.display().fmt(f)
    }
}

impl FromStr for DirDbStore {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(DirDbStore)
    }
}
