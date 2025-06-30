#[tokio::test]
async fn test_insert_and_read_empty_string() {
    insert_and_read(b"").await;
}

#[tokio::test]
async fn test_insert_and_read_hello_world() {
    insert_and_read(b"Hello World!").await;
}

async fn insert_and_read(input: &[u8]) {
    insert_and_read_result(input).await.unwrap()
}

async fn insert_and_read_result(input: &[u8]) -> anyhow::Result<()> {
    use crate::MemStore;
    use pangalactic_store::Store;

    let mut store = MemStore::default();
    let key = store.commit(input).await?;

    let output: Vec<u8> = store.load(&key).await?;

    assert_eq!(input, output.as_slice());
    Ok(())
}
