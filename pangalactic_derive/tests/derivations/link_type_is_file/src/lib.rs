use pangalactic_derivelib::{wrap_derive, LinkHandle, LinkPrim, LinkType};

wrap_derive!(derive_impl);

fn derive_impl(_exec: LinkHandle, input: LinkHandle) -> LinkHandle {
    assert_eq!(input.link_type(), LinkType::File);
    input
}
