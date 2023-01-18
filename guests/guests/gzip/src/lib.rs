use dagwasm_guest::{define_derive, log, ByteWriter, Link, Plan};
use libflate::gzip::Encoder;

#[define_derive]
fn derive_impl(plan: Plan) -> Link {
    let mut r = plan.input.open_file();
    let mut enc = Encoder::new(ByteWriter::open()).unwrap();
    log!("Writing {:?}", &enc.header());
    let copied = std::io::copy(&mut r, &mut enc).unwrap();
    log!("Wrote {copied:?} uncompressed bytes.");
    let w = enc.finish().into_result().unwrap();
    w.commit()
}
