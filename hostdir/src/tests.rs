use crate::HostDirectory;
use pangalactic_hash::Hash;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind;
use pangalactic_serialization::check_serialize_then_deserialize_equality;

type FLDirectory = HostDirectory<Hash>;

#[test]
fn test_empty_directory() -> anyhow::Result<()> {
    check_serialize_then_deserialize_equality(FLDirectory::default())
}

#[test]
fn test_directory() -> anyhow::Result<()> {
    use LinkKind::*;

    let mut d: FLDirectory = FLDirectory::default();
    d.insert(
        "alpha".to_string(),
        Link::new(File, Hash::of(b"a nonempty file")),
    )
    .unwrap();
    d.insert(
        "beta".to_string(),
        Link::new(Dir, Hash::of(b"not a real dir")),
    )
    .unwrap();

    check_serialize_then_deserialize_equality(d)
}
