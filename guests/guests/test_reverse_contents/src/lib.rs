use dagwasm_guest::{define_derive, unwrap, Link, Plan};

#[define_derive]
fn derive_impl(plan: Plan) -> Link {
    reverse_contents(plan.input)
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
                let revname = unwrap!( Result String::from_utf8(bytes) );
                let revchild = reverse_contents(child);
                writer.insert(&revname, revchild);
            }

            writer.commit()
        }
    }
}
