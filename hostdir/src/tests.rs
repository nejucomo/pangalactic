use crate::HostDirectory;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind;
use pangalactic_serialization::check_serialize_then_deserialize_equality;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
struct FakeCID;

type FLDirectory = HostDirectory<FakeCID>;

#[test]
fn test_empty_directory() -> anyhow::Result<()> {
    check_serialize_then_deserialize_equality(FLDirectory::default())
}

#[test]
fn test_directory() -> anyhow::Result<()> {
    use LinkKind::*;

    let mut d: FLDirectory = FLDirectory::default();
    d.insert("alpha".to_string(), Link::new(File, FakeCID))
        .unwrap();
    d.insert("beta".to_string(), Link::new(Dir, FakeCID))
        .unwrap();

    check_serialize_then_deserialize_equality(d)
}
