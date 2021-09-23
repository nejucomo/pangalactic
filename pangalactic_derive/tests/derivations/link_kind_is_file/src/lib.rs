use pangalactic_derivelib::{wrap_derive, LinkHandle, LinkKind, LinkPrim};

wrap_derive!(derive_impl);

fn derive_impl(_exec: LinkHandle, input: LinkHandle) -> LinkHandle {
    assert_eq!(input.kind(), LinkKind::File);
    input
}
