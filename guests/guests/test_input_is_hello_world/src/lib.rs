#[link(wasm_import_module = "dagwasm-host")]
extern "C" {
    fn link_get_kind(handle_link: u64) -> u64;
    fn link_open_file_reader(handle_link: u64) -> u64;
    fn byte_reader_read(handle_byte_reader: u64, ptr: i64, len: u64) -> u64;
    fn byte_reader_close(handle_byte_reader: u64);
    fn link_open_directory_reader(handle_link: u64) -> u64;
    fn directory_reader_has_more_entries(handle_directory_reader: u64) -> u64;
    fn directory_reader_open_name_reader(handle_directory_reader: u64) -> u64;
    fn directory_reader_load_link(handle_directory_reader: u64) -> u64;
    fn directory_reader_next_entry(handle_directory_reader: u64);
}

const TRUE: u64 = 1;
const LEN: usize = 16;

#[no_mangle]
pub extern "C" fn derive(link_plan: u64) -> u64 {
    let link_input = get_input_link(link_plan);

    let byte_reader = unsafe { link_open_file_reader(link_input) };
    let mut buf: [u8; LEN] = [0; LEN];

    let readcnt = read_initial_bytes(byte_reader, &mut buf);

    assert_eq!(&buf[..readcnt], &b"Hello World!"[..]);
    0
}

fn get_input_link(link_plan: u64) -> u64 {
    const LINK_KIND_FILE: u64 = 0;
    const LINK_KIND_DIR: u64 = 1;

    {
        let kind = unsafe { link_get_kind(link_plan) };
        assert_eq!(kind, LINK_KIND_DIR);
    }

    let dir_reader = unsafe { link_open_directory_reader(link_plan) };

    while unsafe { directory_reader_has_more_entries(dir_reader) } == TRUE {
        let reader_name = unsafe { directory_reader_open_name_reader(dir_reader) };

        let mut namebuf: [u8; LEN] = [0; LEN];
        let read_amount = read_initial_bytes(reader_name, &mut namebuf);

        if &namebuf[..read_amount] == &b"input"[..] {
            let link_input = unsafe { directory_reader_load_link(dir_reader) };
            let kind = unsafe { link_get_kind(link_input) };
            assert_eq!(kind, LINK_KIND_FILE);
            return link_input;
        }

        unsafe { directory_reader_next_entry(dir_reader) };
    }

    panic!("No `input` link found.");
}

fn read_initial_bytes(byte_reader: u64, buf: &mut [u8; LEN]) -> usize {
    let read_amount = usize::try_from(unsafe {
        byte_reader_read(
            byte_reader,
            (&mut buf[..]).as_mut_ptr() as i64, // FIXME: safer cast
            u64::try_from(LEN).expect("usize->u64 failure"),
        )
    })
    .expect("u64->usize failure");
    assert!(read_amount <= LEN);

    unsafe { byte_reader_close(byte_reader) };

    read_amount
}
