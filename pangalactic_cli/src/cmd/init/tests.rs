use super::init;
use pangalactic_appdirs::appdirs_init;
use testdir::testdir;

#[test]
fn init_structure() -> std::io::Result<()> {
    use pangalactic_codec::decode_bytes;
    use pangalactic_cryptopubsub::Publisher;
    use pangalactic_fs::file_open;
    use std::io::Read;

    pangalactic_logger::simple_init()?;

    let dirs = appdirs_init!()?;
    let repodir = testdir!().join("repo");
    init(dirs, &repodir)?;

    let pubpath = repodir
        .join(crate::PG_REPO_ATTIC)
        .join("SECRET")
        .join("publisher");
    let mut f = file_open(pubpath)?;
    let mut pbytes = vec![];
    f.read_to_end(&mut pbytes)?;
    let _: Publisher = decode_bytes(&pbytes[..])?;

    Ok(())
}
