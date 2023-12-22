use crate::Link;
use pangalactic_linkkind::LinkKind::*;
use serde::{Deserialize, Serialize};
use test_case::test_case;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct FakeKey(String);

fn fakekey() -> FakeKey {
    FakeKey("fake-key".to_string())
}

type FLink = Link<FakeKey>;

#[test_case(Link::new(File, fakekey()))]
#[test_case(Link::new(Dir, fakekey()))]
fn display_parse_roundtrip(input: FLink) -> anyhow::Result<()> {
    let output: FLink = input.to_string().parse()?;
    assert_eq!(input, output);
    Ok(())
}

#[test_case("file-AAhmYWtlLWtleQ")]
#[test_case("dir-AQhmYWtlLWtleQ")]
fn parse_display_roundtrip(input: &str) -> anyhow::Result<()> {
    let flink: FLink = input.parse()?;
    let output = flink.to_string();
    assert_eq!(input, output);
    Ok(())
}
