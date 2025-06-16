use crate::LinkPath;
use pangalactic_link::testutil::FakeKey;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind::{self, Dir, File};
use pangalactic_name::Path;
use test_case::test_case;

#[test_case(File, "", "pg:F:")]
#[test_case(Dir, "", "pg:D:")]
#[test_case(Dir, "foo", "pg:D:/foo")]
#[test_case(Dir, "foo/bar", "pg:D:/foo/bar")]
fn display(kind: LinkKind, suffix: &str, expected: &str) {
    let path: Path = suffix.parse().unwrap();
    dbg!(&path);
    let link: Link<FakeKey> = Link::new(kind, FakeKey);
    let sp = LinkPath::new(link, path).unwrap();
    let actual = sp.to_string();
    assert_eq!(actual, expected);
}
