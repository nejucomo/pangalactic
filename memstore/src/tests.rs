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
    use dagwasm_blobstore::BlobStore;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut store = MemStore::default();
    let mut w = store.open_writer().await?;
    w.write_all(input).await?;
    let key = store.commit_writer(w).await?;

    let mut r = store.open_reader(key).await?;
    let mut output = vec![];
    r.read_to_end(&mut output).await?;

    assert_eq!(input, output.as_slice());
    Ok(())
}
