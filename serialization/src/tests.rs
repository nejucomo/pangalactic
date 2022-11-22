use crate::testutil::check_serialize_then_deserialize_equality;

#[tokio::test]
async fn test_0u64() {
    check_serialize_then_deserialize_equality(0u64).await;
}

#[tokio::test]
async fn test_0x123456789u64() {
    check_serialize_then_deserialize_equality(0x123456789u64).await;
}

#[tokio::test]
async fn test_0usize() {
    check_serialize_then_deserialize_equality(0usize).await;
}

#[tokio::test]
async fn test_0x123456789usize() {
    check_serialize_then_deserialize_equality(0x123456789usize).await;
}

#[tokio::test]
async fn test_byte_vec() {
    check_serialize_then_deserialize_equality(vec![42u8, 17u8]).await;
}

#[tokio::test]
async fn test_string() {
    check_serialize_then_deserialize_equality("Hello World!".to_string()).await;
}
