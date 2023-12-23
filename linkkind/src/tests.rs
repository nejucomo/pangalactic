use crate::LinkKind::{self, Dir, File};
use pangalactic_unittest_utils::check_display_parse_equivalence;
use test_case::test_case;

#[test_case("file", File)]
#[test_case("dir", Dir)]
fn display_parse_equivalence(text: &str, value: LinkKind) -> anyhow::Result<()> {
    check_display_parse_equivalence(text, value)
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
