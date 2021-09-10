/*
#[link(wasm_import_module = "pangalactic")]
extern "C" {
    fn log(buf: *const u8, len: usize);
}
*/

#[no_mangle]
pub extern "C" fn pangalactic_derive() {
    /*
    let logbytes = "Hello World!".as_bytes();
    unsafe {
        log(logbytes.as_ptr(), logbytes.len());
    }
    */
}
