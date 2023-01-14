use dagwasm_guest::{define_derive, log, ByteWriter, Link, Plan};
use libflate::gzip::Decoder;

#[define_derive]
fn derive_impl(plan: Plan) -> Link {
    let r = plan.input.open_file();
    let mut dec = Decoder::new(r).unwrap();
    log!("Uncompressing {:?}", &dec.header());
    let mut w = ByteWriter::open();
    let copied = std::io::copy(&mut dec, &mut w).unwrap();
    log!("Wrote {copied:?} uncompressed bytes.");
    w.commit()
}
