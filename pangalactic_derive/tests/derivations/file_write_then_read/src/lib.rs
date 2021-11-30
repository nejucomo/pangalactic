use pangalactic_derivelib::{wrap_derive, BufWriterHandle, Kind, LinkHandle, PrimLinkHandle};

wrap_derive!(derive_impl);

fn derive_impl(_exec: LinkHandle, input: LinkHandle) -> LinkHandle {
    let msg = b"Hello World!";

    let w: BufWriterHandle = BufWriterHandle::new();
    w.write(&msg[..]);
    let link = w.commit();

    assert_eq!(link.kind(), Kind::File);

    let mut rbuf = [0u8; 128];
    let r = link.load_file();
    let written = r.read(&mut rbuf[..]);

    assert_eq!(written, msg.len());
    assert_eq!(msg, &rbuf[..written]);

    input
}
