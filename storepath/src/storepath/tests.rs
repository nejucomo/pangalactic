use crate::StorePath;
use pangalactic_link::testutil::{fakekey, FakeStore};
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind::{self, Dir, File};
use test_case::test_case;

#[test_case(File, &[], "file-CGZha2Uta2V5")]
#[test_case(Dir, &[], "dir-CGZha2Uta2V5")]
#[test_case(Dir, &["foo"], "dir-CGZha2Uta2V5/foo")]
#[test_case(Dir, &["foo", "bar"], "dir-CGZha2Uta2V5/foo/bar")]
fn display(kind: LinkKind, suffix: &[&str], expected: &str) {
    let suffix: Vec<_> = suffix.into_iter().map(|s| s.to_string()).collect();
    let link: Link<FakeStore> = Link::new(kind, fakekey());
    let sp = StorePath::new(link, suffix).unwrap();
    let actual = sp.to_string();
    assert_eq!(actual, expected);
}
