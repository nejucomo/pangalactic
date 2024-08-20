use pangalactic_guest::{define_derive, log, unwrap, ByteWriter, Link, Plan};
use libflate::gzip::Decoder;

#[define_derive]
fn derive_impl(plan: Plan) -> Link {
    let r = plan.input.open_file();
    let mut dec = unwrap!( Result Decoder::new(r) );
    log!("Uncompressing {:?}", &dec.header());
    let mut w = ByteWriter::open();
    let copied = unwrap!( Result std::io::copy(&mut dec, &mut w) );
    log!("Wrote {copied:?} uncompressed bytes.");
    w.commit()
}
