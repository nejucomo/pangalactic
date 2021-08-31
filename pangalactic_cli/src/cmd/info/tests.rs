use test_case::test_case;

#[test_case(b".")]
fn info_for_path(pathbytes: &[u8]) -> std::io::Result<()> {
    use crate::cmd::{info, init};
    use crate::repo::Repo;
    use std::io::Cursor;
    use std::path::Path;
    use testdir::testdir;

    let path = &Path::new(std::str::from_utf8(pathbytes).unwrap());
    let repodir = testdir!().join("repo");
    init(&repodir)?;

    let targetpath = repodir.join(path);
    let mut v = vec![];
    info(&mut v, &targetpath)?;

    let repo: Repo = serde_json::from_reader(Cursor::new(v)).unwrap();
    assert_eq!(repo, Repo::from(repodir));
    Ok(())
}
