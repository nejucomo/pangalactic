use dagwasm_guest::bindings::{
    byte_reader_close, byte_reader_read, directory_reader_close, directory_reader_has_more_entries,
    directory_reader_load_link, directory_reader_next_entry, directory_reader_open_name_reader,
    link_get_kind, link_open_directory_reader,
};
use dagwasm_guest::prim::{ByteLen, HandleLink, LINK_KIND_DIR, TRUE};

#[no_mangle]
pub extern "C" fn derive(link_plan: HandleLink) -> HandleLink {
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
                namebuf[..].as_mut_ptr() as i64, // FIXME: safer cast
                ByteLen::try_from(LEN).expect("usize->u64 failure"),
            )
        })
        .expect("u64->usize failure");
        assert!(read_amount <= LEN);

        unsafe { byte_reader_close(reader_name) };

        if namebuf[..read_amount] == b"input"[..] {
            link_input = Some(unsafe { directory_reader_load_link(dir_reader) });
        } else if namebuf[..read_amount] == b"exec"[..] {
            found_exec = true;
        } else {
            panic!("unexpected entry");
        }

        unsafe { directory_reader_next_entry(dir_reader) };
    }
    unsafe { directory_reader_close(dir_reader) };

    if found_exec {
        link_input.expect("no 'input' link found")
    } else {
        panic!("no 'exec' link found");
    }
}
