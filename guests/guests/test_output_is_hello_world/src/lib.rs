use pangalactic_guest::write_bytes;
use pangalactic_guest::{define_derive, Link};

#[define_derive]
fn derive_impl(_: Link) -> Link {
    write_bytes(b"Hello World!")
}
