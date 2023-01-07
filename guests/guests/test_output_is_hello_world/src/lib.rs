use dagwasm_guest::prim::HandleLink;
use dagwasm_guest::write_bytes;

#[no_mangle]
pub extern "C" fn derive(_: HandleLink) -> HandleLink {
    let link = write_bytes(b"Hello World!");
    unsafe { link.unwrap_handle() }
}
