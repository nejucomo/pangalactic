use crate::SecretBoxKey;
use test_case::test_case;

#[test_case(b"")]
#[test_case(b"hello world")]
#[test_case(b"\x00")]
fn seal_open(b: &[u8]) {
    rust_sodium::init().unwrap();

    let k = SecretBoxKey::generate();
    let ciphertext = k.seal(b);
    let b2 = k.open(&ciphertext[..]).unwrap();
    assert_eq!(b, &b2[..]);
}
