#[link(wasm_import_module = "pangalactic")]
extern {
    fn phone_home();

    // fn get_bytes(buf: *mut u8, len: usize);
}

#[no_mangle]
pub extern fn main() {
    unsafe {
        phone_home();
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
