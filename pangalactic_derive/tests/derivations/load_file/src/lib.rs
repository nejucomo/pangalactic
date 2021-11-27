use pangalactic_derivelib::{wrap_derive, BufReaderHandle, Kind, LinkHandle, PrimLinkHandle};

wrap_derive!(derive_impl);

fn derive_impl(_exec: LinkHandle, input: LinkHandle) -> LinkHandle {
    assert_eq!(input.kind(), Kind::File);
    let _: BufReaderHandle = input.load_file();
    input
}
