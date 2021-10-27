use pangalactic_derivelib::{wrap_derive, Kind, LinkHandle, LinkPrim, ReadHandle};

wrap_derive!(derive_impl);

fn derive_impl(_exec: LinkHandle, input: LinkHandle) -> LinkHandle {
    assert_eq!(input.kind(), Kind::File);
    let _: ReadHandle = input.read_file();
    input
}
