use crate::Directory;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind;
use pangalactic_serialization::testutil::check_serialize_then_deserialize_equality;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct FakeCID;

type FLDirectory = Directory<Link<FakeCID>>;

#[test]
fn test_empty_directory() {
    check_serialize_then_deserialize_equality::<FLDirectory>(Directory::default());
}

#[test]
fn test_directory() {
    use LinkKind::*;

    let mut d: FLDirectory = Directory::default();
    d.insert("alpha".to_string(), Link::new(File, FakeCID))
        .unwrap();
    d.insert("beta".to_string(), Link::new(Dir, FakeCID))
        .unwrap();

    check_serialize_then_deserialize_equality::<FLDirectory>(d);
}
