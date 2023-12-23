use crate::StorePath;
use pangalactic_link::testutil::fakekey;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind::{self, Dir, File};
use pangalactic_unittest_utils::check_display_parse_equivalence;
use test_case::test_case;

#[test_case("file-CGZha2Uta2V5", File, &[])]
#[test_case("dir-CGZha2Uta2V5", Dir, &[])]
#[test_case("dir-CGZha2Uta2V5/foo", Dir, &["foo"])]
#[test_case("dir-CGZha2Uta2V5/foo/bar", Dir, &["foo", "bar"])]
fn display_parse_equivalence(text: &str, kind: LinkKind, suffix: &[&str]) -> anyhow::Result<()> {
    let suffix: Vec<_> = suffix.into_iter().map(|s| s.to_string()).collect();
    let link = Link::new(kind, fakekey());
    let sp = StorePath::try_from((link, suffix))?;
    check_display_parse_equivalence(text, sp)?;
    Ok(())
}
