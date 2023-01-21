use dagwasm_guest::{define_derive, ByteReader, ByteWriter, Link, Plan};

#[define_derive]
fn derive_impl(plan: Plan) -> Link {
    let mut builder = tar::Builder::new(ByteWriter::open());
    builder.mode(tar::HeaderMode::Deterministic);

    append_link(&mut builder, plan.input, "");

    builder.into_inner().unwrap().commit()
}

fn append_link(builder: &mut tar::Builder<ByteWriter>, link: Link, path: &str) {
    use dagwasm_guest::Reader::{Dir, File};

    match link.open() {
        File(r) => append_file(builder, r, path),
        Dir(d) => {
            for (name, link) in d {
                append_link(builder, link, &format!("{}/{}", path, name));
            }
        }
    }
}

fn append_file(builder: &mut tar::Builder<ByteWriter>, file: ByteReader, path: &str) {
    let contents = file.read_to_vec();
    builder
        .append(&make_header(&path, contents.len()), contents.as_slice())
        .unwrap();
}

fn make_header(path: &str, length: usize) -> tar::Header {
    let mut header = tar::Header::new_gnu();
    header.set_path(&path).unwrap();
    header.set_size(u64::try_from(length).expect("usize->u64 conversion failure"));
    header.set_cksum();
    header
}
