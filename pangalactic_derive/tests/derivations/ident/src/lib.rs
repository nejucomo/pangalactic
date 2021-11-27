use pangalactic_derivelib::{wrap_derive, LinkHandle, PrimLinkHandle};

wrap_derive!(derive_impl);

fn derive_impl(_exec: LinkHandle, input: LinkHandle) -> LinkHandle {
    input
}
