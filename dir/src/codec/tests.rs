use crate::codec::{AsyncDeserialize, AsyncSerialize};

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
