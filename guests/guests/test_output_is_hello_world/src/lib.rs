use dagwasm_guest::write_bytes;
use dagwasm_guest::{define_derive, Link};

#[define_derive]
fn derive_impl(_: Link) -> Link {
    write_bytes(b"Hello World!")
}
