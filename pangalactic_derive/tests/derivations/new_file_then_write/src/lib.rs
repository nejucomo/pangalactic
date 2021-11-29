use pangalactic_derivelib::{wrap_derive, BufWriterHandle, LinkHandle, PrimLinkHandle};

wrap_derive!(derive_impl);

fn derive_impl(_exec: LinkHandle, input: LinkHandle) -> LinkHandle {
    let msg = b"Hello World!";

    let h: BufWriterHandle = BufWriterHandle::new();
    h.write(&msg[..]);
    input
}
