use crate::SigningPair;
use test_case::test_case;

#[test_case(b"")]
#[test_case(b"\x00")]
#[test_case(b"hello world")]
fn pos_sign_verify(msg: &[u8]) -> std::io::Result<()> {
    pangalactic_logger::simple_init()?;

    let pair = SigningPair::generate();
    let signed = pair.signer.sign(msg);
    let msg2 = pair.verifier.verify(&signed[..]).unwrap();
    assert_eq!(msg, &msg2[..]);

    Ok(())
}
