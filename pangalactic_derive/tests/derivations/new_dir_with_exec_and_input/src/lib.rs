use pangalactic_derivelib::{wrap_derive, DirWriterHandle, LinkHandle, PrimLinkHandle};

wrap_derive!(derive_impl);

fn derive_impl(exec: LinkHandle, input: LinkHandle) -> LinkHandle {
    let h = DirWriterHandle::new();
    h.add_link("exec", exec);
    h.add_link("input", input);
    h.commit()
}
