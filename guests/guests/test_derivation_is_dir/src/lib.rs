#[link(wasm_import_module = "dagwasm_host")]
extern "C" {
    fn link_get_kind(handle_link: u64) -> u64;
}

const LINK_KIND_DIR: u64 = 2;

#[no_mangle]
pub extern "C" fn derive(derivation: u64) -> u64 {
    let kind = unsafe { link_get_kind(derivation) };
    assert_eq!(kind, LINK_KIND_DIR);
    0
}
