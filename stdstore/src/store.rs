use anyhow::Result;
use async_trait::async_trait;
use pangalactic_hash::Hash;
use pangalactic_iowrappers::Readable;
use pangalactic_layer_cidmeta::{CidMeta, CidMetaLayer};
use pangalactic_path::{PathLayer, StorePath, ViaPath};
use pangalactic_store::{Commit, Load, Store};
use pangalactic_store_dirdb::DirDbStore;

#[derive(Debug, Default)]
pub struct StandardStore(Inner);

type Inner = PathLayer<CidMetaLayer<DirDbStore>>;

pub type StandardPath = StorePath<CidMeta<Hash>>;
pub type StandardReader = ViaPath<Readable<Readable<tokio::fs::File>>>;
pub type StandardWriter = ViaPath<
    pangalactic_hostdir::Writer<pangalactic_layer_cidmeta::Writer<pangalactic_store_dirdb::Writer>>,
>;

#[async_trait]
impl Store for StandardStore {
    type CID = StandardPath;
    type Reader = StandardReader;
    type Writer = StandardWriter;

    async fn open_writer(&self) -> Result<Self::Writer> {
        self.0.open_writer().await
    }
}

#[async_trait]
impl Commit<StandardStore> for StandardWriter {
    async fn commit_into_store(self, store: &mut StandardStore) -> Result<StandardPath> {
        store.0.commit(self).await
    }
}

#[async_trait]
impl Load<StandardStore> for StandardReader {
    async fn load_from_store(store: &StandardStore, path: &StandardPath) -> Result<Self> {
        Self::load_from_store(&store.0, path).await
    }
}
