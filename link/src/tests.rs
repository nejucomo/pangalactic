use crate::testutil::{FakeKey, FakeStore};
use crate::Link;
use pangalactic_linkkind::LinkKind::{self, Dir, File};
use test_case::test_case;

#[test_case(File, "pg:file-<FakeStore>-")]
#[test_case(Dir, "pg:dir-<FakeStore>-")]
fn display(kind: LinkKind, expected: &str) {
    let link: Link<FakeStore> = Link::new(kind, FakeKey);
    let actual = link.to_string();
    assert_eq!(actual, expected);
}

#[test_case(File)]
#[test_case(Dir)]
fn display_parse_roundtrip(kind: LinkKind) -> anyhow::Result<()> {
    let input: Link<FakeStore> = Link::new(kind, FakeKey);
    let output: Link<FakeStore> = input.to_string().parse()?;
    assert_eq!(input, output);
    Ok(())
}

#[test_case("pg:file-<FakeStore>-")]
#[test_case("pg:dir-<FakeStore>-")]
fn parse_display_roundtrip(input: &str) -> anyhow::Result<()> {
    let link: Link<FakeStore> = input.parse()?;
    let output = link.to_string();
    assert_eq!(input, output);
    Ok(())
}
