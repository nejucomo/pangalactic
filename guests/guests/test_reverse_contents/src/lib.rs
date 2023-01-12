use dagwasm_guest::prim::HandleLink;
use dagwasm_guest::Link;

#[no_mangle]
pub extern "C" fn prim_derive_impl(primplan: HandleLink) -> HandleLink {
    let plan = unsafe { Link::wrap_handle(primplan) };
    let input = plan.open_directory().select_entry("input");
    let output = reverse_contents(input);
    unsafe { output.unwrap_handle() }
}

fn reverse_contents(link: Link) -> Link {
    use dagwasm_guest::Reader::*;

    match link.open() {
        File(reader) => {
            let mut bytes = reader.read_to_vec();
            bytes.reverse();
            dagwasm_guest::write_bytes(&bytes)
        }
        Dir(reader) => {
            use dagwasm_guest::DirectoryWriter;

            let writer = DirectoryWriter::open();
            for (name, child) in reader {
                let mut bytes = name.into_bytes();
                bytes.reverse();
                // Note: Cannot handle non-ascii names:
                let revname = String::from_utf8(bytes).unwrap();
                let revchild = reverse_contents(child);
                writer.insert(&revname, revchild);
            }

            writer.commit()
        }
    }
}
