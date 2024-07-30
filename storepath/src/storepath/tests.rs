use crate::StorePath;
use pangalactic_link::testutil::FakeKey;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind::{self, Dir, File};
use test_case::test_case;

#[test_case(File, &[], "pg:file-<FakeStore>-")]
#[test_case(Dir, &[], "pg:dir-<FakeStore>-")]
#[test_case(Dir, &["foo"], "pg:dir-<FakeStore>-/foo")]
#[test_case(Dir, &["foo", "bar"], "pg:dir-<FakeStore>-/foo/bar")]
fn display(kind: LinkKind, suffix: &[&str], expected: &str) {
    let suffix: Vec<_> = suffix.into_iter().map(|s| s.to_string()).collect();
    let link: Link<FakeKey> = Link::new(kind, FakeKey);
    let sp = StorePath::new(link, suffix).unwrap();
    let actual = sp.to_string();
    assert_eq!(actual, expected);
}
