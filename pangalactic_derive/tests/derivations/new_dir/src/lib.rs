use pangalactic_derivelib::{wrap_derive, DirWriterHandle, LinkHandle, PrimLinkHandle};

wrap_derive!(derive_impl);

fn derive_impl(_exec: LinkHandle, input: LinkHandle) -> LinkHandle {
    let _ = DirWriterHandle::new();
    input
}
