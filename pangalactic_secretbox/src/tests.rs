use crate::SecretBoxKey;
use test_case::test_case;

#[test_case(b"")]
#[test_case(b"hello world")]
#[test_case(b"\x00")]
fn seal_open(b: &[u8]) -> std::io::Result<()> {
    pangalactic_logger::test_init();

    let k = SecretBoxKey::generate();
    let ciphertext = k.seal(b);
    let b2 = k.open(&ciphertext[..]).unwrap();
    let b3 = k.open(ciphertext).unwrap();
    assert_eq!(b, &b2[..]);
    assert_eq!(b2, b3);

    Ok(())
}
