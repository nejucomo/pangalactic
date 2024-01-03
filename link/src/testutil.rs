use std::{fmt::Display, str::FromStr};

use async_trait::async_trait;
use pangalactic_store::{Store, StoreCid};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct FakeKey;

impl StoreCid for FakeKey {}

impl Display for FakeKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "<FAKE-KEY>".fmt(f)
    }
}

impl FromStr for FakeKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "<FAKE-KEY>" {
            Ok(Self)
        } else {
            anyhow::bail!("expected `<FAKE-KEY>`")
        }
    }
}

#[derive(Debug)]
pub struct FakeStore;

#[async_trait]
impl Store for FakeStore {
    const SCHEME: &'static str = "FAKE";

    type CID = FakeKey;
    type Reader = tokio::io::Empty;
    type Writer = tokio::io::Sink;

    async fn open_reader(&mut self, _: &Self::CID) -> anyhow::Result<Self::Reader> {
        unimplemented!("open_reader")
    }

    async fn open_writer(&mut self) -> anyhow::Result<Self::Writer> {
        unimplemented!("open_writer")
    }

    async fn commit_writer(&mut self, _: Self::Writer) -> anyhow::Result<Self::CID> {
        unimplemented!("commit_writer")
    }
}
