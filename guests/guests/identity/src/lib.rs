#[link(wasm_import_module = "dagwasm-host")]
extern "C" {
    fn link_get_kind(handle_link: u64) -> u64;
    fn link_open_directory_reader(handle_link: u64) -> u64;
    fn directory_reader_has_more_entries(handle_directory_reader: u64) -> u64;
    fn directory_reader_open_name_reader(handle_directory_reader: u64) -> u64;
    fn directory_reader_load_link(handle_directory_reader: u64) -> u64;
    fn directory_reader_next_entry(handle_directory_reader: u64);
    fn byte_reader_read(handle_byte_reader: u64, ptr: i64, len: u64) -> u64;
    fn byte_reader_close(handle_byte_reader: u64);
}

const TRUE: u64 = 1;
const LINK_KIND_DIR: u64 = 1;

#[no_mangle]
pub extern "C" fn derive(link_plan: u64) -> u64 {
    {
        let kind = unsafe { link_get_kind(link_plan) };
        assert_eq!(kind, LINK_KIND_DIR);
    }

    let dir_reader = unsafe { link_open_directory_reader(link_plan) };

    let mut link_input = None;
    let mut found_exec = false;
    while unsafe { directory_reader_has_more_entries(dir_reader) } == TRUE {
        let reader_name = unsafe { directory_reader_open_name_reader(dir_reader) };

        const LEN: usize = 16;
        let mut namebuf: [u8; LEN] = [0; LEN];

        let read_amount = usize::try_from(unsafe {
            byte_reader_read(
                reader_name,
                (&mut namebuf[..]).as_mut_ptr() as i64, // FIXME: safer cast
                u64::try_from(LEN).expect("usize->u64 failure"),
            )
        })
        .expect("u64->usize failure");
        assert!(read_amount <= LEN);

        unsafe { byte_reader_close(reader_name) };

        if &namebuf[..read_amount] == &b"input"[..] {
            link_input = Some(unsafe { directory_reader_load_link(dir_reader) });
        } else if &namebuf[..read_amount] == &b"exec"[..] {
            found_exec = true;
        } else {
            panic!("unexpected entry");
        }

        unsafe { directory_reader_next_entry(dir_reader) };
    }

    if found_exec {
        link_input.expect("no 'input' link found")
    } else {
        panic!("no 'exec' link found");
    }
}
