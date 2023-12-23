use crate::testutil::fakekey;
use crate::Link;
use pangalactic_linkkind::LinkKind::{self, Dir, File};
use pangalactic_serialization::check_serialize_then_deserialize_equality;
use pangalactic_unittest_utils::check_display_parse_equivalence;
use test_case::test_case;

#[test_case("file-CGZha2Uta2V5", File)]
#[test_case("dir-CGZha2Uta2V5", Dir)]
fn display_parse_equivalence(text: &str, kind: LinkKind) -> anyhow::Result<()> {
    check_display_parse_equivalence(text, Link::new(kind, fakekey()))
}

#[test_case(File)]
#[test_case(Dir)]
fn serialize_then_deserialize_equality(kind: LinkKind) -> anyhow::Result<()> {
    check_serialize_then_deserialize_equality(Link::new(kind, fakekey()))
}
