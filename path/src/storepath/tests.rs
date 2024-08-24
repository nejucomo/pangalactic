use crate::StorePath;
use pangalactic_link::testutil::FakeKey;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind::{self, Dir, File};
use pangalactic_name::Name;
use test_case::test_case;

#[test_case(File, [], "pg:F:")]
#[test_case(Dir, [], "pg:D:")]
#[test_case(Dir, ["foo"], "pg:D:/foo")]
#[test_case(Dir, ["foo", "bar"], "pg:D:/foo/bar")]
fn display<const K: usize>(kind: LinkKind, suffix: [&'static str; K], expected: &str) {
    let suffix: Vec<_> = suffix.into_iter().map(Name::from_static).collect();
    let link: Link<FakeKey> = Link::new(kind, FakeKey);
    let sp = StorePath::new(link, suffix).unwrap();
    let actual = sp.to_string();
    assert_eq!(actual, expected);
}
