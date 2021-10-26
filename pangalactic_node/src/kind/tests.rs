use crate::{Kind, UnknownKindEncoding};
use std::convert::TryFrom;
use test_case::test_case;

#[test_case(Kind::File)]
#[test_case(Kind::Dir)]
fn codec(k: Kind) {
    let u = i64::from(k);
    let k2 = Kind::try_from(u).unwrap();
    assert_eq!(k, k2);
    let u2 = i64::from(k2);
    assert_eq!(u, u2);
}

#[test]
fn unknown_encoding() {
    let bad = 0x4895349854;
    assert_eq!(UnknownKindEncoding(bad), Kind::try_from(bad).unwrap_err());
}
