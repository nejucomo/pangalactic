use crate::Directory;
use async_trait::async_trait;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind;
use pangalactic_serialization::testutil::check_serialize_then_deserialize_equality;
use pangalactic_serialization::{AsyncDeserialize, AsyncSerialize};
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, PartialEq, Eq)]
struct FakeCID;
type FLDirectory = Directory<Link<FakeCID>>;

#[async_trait]
impl AsyncSerialize for FakeCID {
    async fn write_into<W>(&self, _w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        Ok(())
    }
}

#[async_trait]
impl AsyncDeserialize for FakeCID {
    async fn read_from<R>(_r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        Ok(FakeCID)
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
    d.insert("alpha".to_string(), Link::new(File, FakeCID))
        .unwrap();
    d.insert("beta".to_string(), Link::new(Dir, FakeCID))
        .unwrap();

    check_serialize_then_deserialize_equality::<FLDirectory>(d).await;
}
