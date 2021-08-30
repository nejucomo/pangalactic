use super::init;
use pangalactic_appdirs::appdirs_init;
use testdir::testdir;

#[test]
fn init_structure() -> std::io::Result<()> {
    use pangalactic_codecpath::CodecPath;
    use pangalactic_cryptopubsub::Publisher;

    pangalactic_logger::simple_init()?;

    let dirs = appdirs_init!()?;
    let repodir = testdir!().join("repo");
    init(dirs, &repodir)?;

    let pubpath = repodir
        .join(crate::PG_REPO_ATTIC)
        .join("SECRET")
        .join("publisher");

    pubpath.decode_contents::<Publisher>()?;

    Ok(())
}
