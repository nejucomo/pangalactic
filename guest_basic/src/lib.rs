#[link(wasm_import_module = "pangalactic")]
extern "C" {
    fn phone_home();

    fn log(buf: *const u8, len: usize);

    // fn get_bytes(buf: *mut u8, len: usize);
}

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        phone_home();
    }

    let logbytes = "Hello World!".as_bytes();
    unsafe {
        log(logbytes.as_ptr(), logbytes.len());
    }

    /*
    let mut v = Vec::with_capacity(8);

    for _ in 0..8 {
        v.push(0u8);
    }

    unsafe {
        get_bytes(v.as_mut_ptr(), 8);
    }
    */
}
