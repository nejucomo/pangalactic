use async_trait::async_trait;
use pangalactic_store::{Store, StoreCid};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct FakeKey;

impl StoreCid for FakeKey {
    fn transport_scheme() -> String {
        "<FakeKey>".to_string()
    }
}

#[derive(Debug)]
pub struct FakeStore;

#[async_trait]
impl Store for FakeStore {
    type CID = FakeKey;
    type Reader = tokio::io::Empty;
    type Writer = tokio::io::Sink;

    async fn open_reader(&self, _: &Self::CID) -> anyhow::Result<Self::Reader> {
        unimplemented!("open_reader")
    }

    async fn open_writer(&self) -> anyhow::Result<Self::Writer> {
        unimplemented!("open_writer")
    }

    async fn commit_writer(&mut self, _: Self::Writer) -> anyhow::Result<Self::CID> {
        unimplemented!("commit_writer")
    }
}
