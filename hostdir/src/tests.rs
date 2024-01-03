use crate::HostDirectory;
use pangalactic_hash::Hash;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind::*;
use pangalactic_serialization::check_serialize_then_deserialize_equality;
use pangalactic_store_mem::MemStore;

type FLDirectory = HostDirectory<MemStore>;

#[test]
fn test_empty_directory() -> anyhow::Result<()> {
    check_serialize_then_deserialize_equality(FLDirectory::default())
}

#[test]
fn test_directory() -> anyhow::Result<()> {
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

// This is an odd place for this test since it is not focused on
// `hostdir` specifically. Instead this is the best spot to expose a bug in
// `Link<CidMetaLayer<MemStore>>` roundtrip `Display/FromStr` because if
// `CidMetaLayer` uses `-` to separate fields and a `Hash` also contains
// `-` there is parser ambiguity.
#[tokio::test]
async fn test_cidmeta_has_display_fromstr_roundtrip_hyphen_safety() -> anyhow::Result<()> {
    use pangalactic_store::Store;

    type ComposedStore = pangalactic_layer_cidmeta::CidMetaLayer<MemStore>;

    let mut store = ComposedStore::default();
    // This hash contains `-`:
    let cid = store
        .write(&[02u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8])
        .await?;
    let link_input: Link<ComposedStore> = Link::new(File, cid);
    let link_output = link_input.to_string().parse()?;
    assert_eq!(link_input, link_output);
    Ok(())
}
