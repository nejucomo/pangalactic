use crate::{Directory, Link, LinkKind};
use async_trait::async_trait;
use dagwasm_serialization::testutil::check_serialize_then_deserialize_equality;
use dagwasm_serialization::{AsyncDeserialize, AsyncSerialize};
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, PartialEq, Eq)]
struct FakeKey;
type FLDirectory = Directory<FakeKey>;

#[async_trait]
impl AsyncSerialize for FakeKey {
    async fn write_into<W>(&self, _w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        Ok(())
    }
}

#[async_trait]
impl AsyncDeserialize for FakeKey {
    async fn read_from<R>(_r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        Ok(FakeKey)
    }
}

#[tokio::test]
async fn test_empty_directory() {
    check_serialize_then_deserialize_equality::<FLDirectory>(Directory::default()).await;
}

#[tokio::test]
async fn test_directory() {
    use LinkKind::*;

    let mut d: FLDirectory = Directory::default();
    d.insert("alpha".to_string(), Link::new(File, FakeKey))
        .unwrap();
    d.insert("beta".to_string(), Link::new(Dir, FakeKey))
        .unwrap();

    check_serialize_then_deserialize_equality::<FLDirectory>(d).await;
}
