use pangalactic_guest::{define_derive, log, unwrap, ByteReader, ByteWriter, Link, Plan};

#[define_derive]
fn derive_impl(plan: Plan) -> Link {
    let mut builder = tar::Builder::new(ByteWriter::open());
    builder.mode(tar::HeaderMode::Deterministic);

    append_link(&mut builder, plan.input, "");

    unwrap!(Result builder.into_inner()).commit()
}

fn append_link(builder: &mut tar::Builder<ByteWriter>, link: Link, path: &str) {
    use pangalactic_guest::Reader::{Dir, File};

    match link.open() {
        File(r) => append_file(builder, r, path, link.node_size()),
        Dir(d) => {
            for (name, link) in d {
                append_link(
                    builder,
                    link,
                    &(if path.is_empty() {
                        name.take()
                    } else {
                        format!("{path}/{name}")
                    }),
                );
            }
        }
    }
}

fn append_file(builder: &mut tar::Builder<ByteWriter>, file: ByteReader, path: &str, size: usize) {
    unwrap!(
        Result
        builder
            .append(&make_header(path, size), file)
    );
}

fn make_header(path: &str, length: usize) -> tar::Header {
    let mut header = tar::Header::new_gnu();
    log!("make_header({path:?})");
    unwrap!(Result header.set_path(path));
    header.set_size(u64::try_from(length).expect("usize->u64 conversion failure"));
    header.set_cksum();
    header
}
