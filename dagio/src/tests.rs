use crate::Dagio;
use pangalactic_hostdir::HostDirectory;
use pangalactic_store_mem::MemStore;

#[tokio::test]
async fn check_empty_file_link_transport_encoding() -> anyhow::Result<()> {
    let expected = "pg:file-metab3";
    let mut dagio = Dagio::from(MemStore::default());
    let link = dagio.commit(b"".as_slice()).await?;
    let linktext = link.to_string();
    let (found, _) = linktext.rsplit_once('-').unwrap();
    assert_eq!(found, expected);
    Ok(())
}

#[tokio::test]
async fn insert_file_and_read_result() -> anyhow::Result<()> {
    let input = b"Hello World!";

    let mut dagio = Dagio::from(MemStore::default());
    let link = dagio.commit(input.as_slice()).await?;
    dbg!(&link);

    let output: Vec<u8> = dagio.load(&link).await?;

    assert_eq!(input, output.as_slice());
    Ok(())
}

#[tokio::test]
async fn insert_empty_directory_and_read_result() -> anyhow::Result<()> {
    let input = HostDirectory::default();

    let mut dagio = Dagio::from(MemStore::default());
    let link = dagio.commit(input.clone()).await?;
    dbg!(&link);

    let output: HostDirectory<_> = dagio.load(&link).await?;
    dbg!(&input, &output);

    assert_eq!(input, output);
    Ok(())
}

#[tokio::test]
async fn insert_singleton_directory_and_read_result() -> anyhow::Result<()> {
    let input_hw = b"Hello World!";

    let mut dagio = Dagio::from(MemStore::default());

    let link_hw = dagio.commit(input_hw.as_slice()).await?;
    dbg!(&link_hw);

    let input_dir = HostDirectory::from_iter([("hello.txt", link_hw)]);
    let link_dir = dagio.commit(input_dir.clone()).await?;

    let output_dir: HostDirectory<_> = dagio.load(&link_dir).await?;
    assert_eq!(input_dir, output_dir);

    let outlink_hw = output_dir.get("hello.txt").unwrap();
    let output_hw: Vec<u8> = dagio.load(outlink_hw).await?;

    assert_eq!(input_hw, output_hw.as_slice());
    Ok(())
}
