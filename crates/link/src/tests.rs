use crate::testutil::FakeKey;
use crate::Link;
use pangalactic_linkkind::LinkKind::{self, Dir, File};
use test_case::test_case;

#[test_case(File, "pg:F:")]
#[test_case(Dir, "pg:D:")]
fn display(kind: LinkKind, expected: &str) {
    let link: Link<FakeKey> = Link::new(kind, FakeKey);
    let actual = link.to_string();
    assert_eq!(actual, expected);
}

#[test_case(File)]
#[test_case(Dir)]
fn display_parse_roundtrip(kind: LinkKind) -> anyhow::Result<()> {
    let input: Link<FakeKey> = Link::new(kind, FakeKey);
    let output: Link<FakeKey> = input.to_string().parse()?;
    assert_eq!(input, output);
    Ok(())
}

#[test_case("pg:F:")]
#[test_case("pg:D:")]
fn parse_display_roundtrip(input: &str) -> anyhow::Result<()> {
    let link: Link<FakeKey> = input.parse()?;
    let output = link.to_string();
    assert_eq!(input, output);
    Ok(())
}
