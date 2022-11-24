use crate::Dagio;
use dagwasm_memstore::MemStore;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn insert_and_read_result() -> anyhow::Result<()> {
    let input = b"Hello World!";

    let mut dagio = Dagio::from(MemStore::default());
    let mut w = dagio.open_file_writer().await?;
    w.write_all(input).await?;
    let link = dagio.commit_file_writer(w).await?;
    dbg!(&link);

    let mut r = dagio.open_file_reader(link).await?;
    let mut output = vec![];
    r.read_to_end(&mut output).await?;

    assert_eq!(input, output.as_slice());
    Ok(())
}
