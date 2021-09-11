#[link(wasm_import_module = "pangalactic")]
extern "C" {
    fn link_type(handle: i64) -> i64;
}

#[no_mangle]
pub extern "C" fn derive(_exec: i64, input: i64) -> i64 {
    assert_eq!(unsafe { link_type(input) }, 0);
    input
}
