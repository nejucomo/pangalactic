use crate::testutil::{fakekey, FakeKey};
use crate::Link;
use pangalactic_linkkind::LinkKind::{self, Dir, File};
use test_case::test_case;

#[test_case(File, "file-CGZha2Uta2V5")]
#[test_case(Dir, "dir-CGZha2Uta2V5")]
fn display(kind: LinkKind, expected: &str) {
    let link = Link::new(kind, fakekey());
    let actual = link.to_string();
    assert_eq!(actual, expected);
}

#[test_case(File)]
#[test_case(Dir)]
fn display_parse_roundtrip(kind: LinkKind) -> anyhow::Result<()> {
    let input = Link::new(kind, fakekey());
    let output: Link<FakeKey> = input.to_string().parse()?;
    assert_eq!(input, output);
    Ok(())
}

#[test_case("file-CGZha2Uta2V5")]
#[test_case("dir-CGZha2Uta2V5")]
fn parse_display_roundtrip(input: &str) -> anyhow::Result<()> {
    let flink: Link<FakeKey> = input.parse()?;
    let output = flink.to_string();
    assert_eq!(input, output);
    Ok(())
}
