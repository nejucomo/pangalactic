use super::init;
use pangalactic_appdirs::appdirs_init;
use testdir::testdir;

#[test]
fn init_structure() -> std::io::Result<()> {
    pangalactic_logger::simple_init()?;

    let dirs = appdirs_init!()?;
    let repodir = testdir!().join("repo");
    init(dirs, &repodir)?;
    assert!(repodir.join(crate::PG_REPO_ATTIC).is_dir());

    Ok(())
}
