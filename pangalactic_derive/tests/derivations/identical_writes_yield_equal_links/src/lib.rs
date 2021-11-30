use pangalactic_derivelib::{wrap_derive, BufWriterHandle, LinkHandle, PrimLinkHandle};

wrap_derive!(derive_impl);

fn derive_impl(_exec: LinkHandle, input: LinkHandle) -> LinkHandle {
    let msg = b"Hello World!";
    let mut links = vec![];

    for _ in 0..2 {
        let h: BufWriterHandle = BufWriterHandle::new();
        h.write(&msg[..]);
        links.push(h.commit());
    }

    assert_eq!(links[0], links[1]);
    input
}
