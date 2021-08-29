use crate::Publisher;
use test_case::test_case;

#[test_case(b"")]
#[test_case(b"\x00")]
#[test_case(b"hello world")]
fn distributor_unwrap(msg: &[u8]) {
    let p = Publisher::generate();
    let publication = p.publish(0, msg);
    let contents = p.distributor().unwrap(&publication).unwrap();
    assert_eq!(contents.sequence, 0);
    assert_ne!(msg, &contents.data[..]);
}

#[test_case(b"")]
#[test_case(b"\x00")]
#[test_case(b"hello world")]
fn subscriber_unwrap(msg: &[u8]) {
    let p = Publisher::generate();
    let publication = p.publish(0, msg);
    let contents = p.subscriber().unwrap(&publication).unwrap();
    assert_eq!(contents.sequence, 0);
    assert_eq!(msg, &contents.data[..]);
}
