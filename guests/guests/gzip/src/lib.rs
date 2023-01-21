use dagwasm_guest::{define_derive, log, unwrap, ByteWriter, Link, Plan};
use libflate::gzip::Encoder;

#[define_derive]
fn derive_impl(plan: Plan) -> Link {
    let mut r = plan.input.open_file();
    let mut enc = unwrap!( Result Encoder::new(ByteWriter::open()) );
    log!("Writing {:?}", &enc.header());
    let copied = unwrap!( Result std::io::copy(&mut r, &mut enc) );
    log!("Wrote {copied:?} uncompressed bytes.");
    let w = unwrap!( Result enc.finish().into_result() );
    w.commit()
}
