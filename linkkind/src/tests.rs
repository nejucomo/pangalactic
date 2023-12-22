use crate::LinkKind::{self, Dir, File};
use test_case::test_case;

#[test_case(File)]
#[test_case(Dir)]
fn display_parse_roundtrip(input: LinkKind) -> anyhow::Result<()> {
    let output: LinkKind = input.to_string().parse()?;
    assert_eq!(input, output);
    Ok(())
}

#[test_case("file")]
#[test_case("dir")]
fn parse_display_roundtrip(input: &str) -> anyhow::Result<()> {
    let val: LinkKind = input.parse()?;
    let output = val.to_string();
    assert_eq!(input, &output);
    Ok(())
}

#[test_case(
    "f1le",
    r#"unrecognized LinkKind "f1le", expected one of: "file", "dir""#
)]
#[test_case(
    "dire",
    r#"unrecognized LinkKind "dire", expected one of: "file", "dir""#
)]
fn parse_error(input: &str, expected_error: &str) {
    let emsg = input.parse::<LinkKind>().err().unwrap().to_string();
    assert_eq!(emsg, expected_error);
}
