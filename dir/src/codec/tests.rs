use crate::codec::{AsyncDeserialize, AsyncSerialize};
use crate::{Directory, Link, LinkKind};
use async_trait::async_trait;
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

#[tokio::test]
async fn test_0u64() {
    serialize_then_deserialize(0u64).await;
}

#[tokio::test]
async fn test_0x123456789u64() {
    serialize_then_deserialize(0x123456789u64).await;
}

#[tokio::test]
async fn test_0usize() {
    serialize_then_deserialize(0usize).await;
}

#[tokio::test]
async fn test_0x123456789usize() {
    serialize_then_deserialize(0x123456789usize).await;
}

#[tokio::test]
async fn test_byte_vec() {
    serialize_then_deserialize(vec![42u8, 17u8]).await;
}

#[tokio::test]
async fn test_string() {
    serialize_then_deserialize("Hello World!".to_string()).await;
}

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
    serialize_then_deserialize::<FLDirectory>(Directory::default()).await;
}

#[tokio::test]
async fn test_directory() {
    use LinkKind::*;

    let mut d: FLDirectory = Directory::default();
    d.insert("alpha".to_string(), Link::new(File, FakeKey))
        .unwrap();
    d.insert("beta".to_string(), Link::new(Dir, FakeKey))
        .unwrap();

    serialize_then_deserialize::<FLDirectory>(d).await;
}

async fn serialize_then_deserialize<T>(input: T)
where
    T: AsyncSerialize + AsyncDeserialize + PartialEq + std::fmt::Debug,
{
    let mut buf = vec![];

    dbg!(&input);
    input.write_into(&mut buf).await.unwrap();
    dbg!(&buf);
    let output = T::read_from(buf.as_slice()).await.unwrap();

    assert_eq!(input, output);
}
