use crate::flexint::FlexIntEncoding;
use test_case::test_case;

#[test_case(0x00, &[0x00])]
#[test_case(0x01, &[0x01])]
#[test_case(0x7f, &[0x7f])]
#[test_case(0x80, &[0x80, 0x01])]
#[test_case(0x3fff, &[0xff, 0x7f])]
#[test_case(0x4000, &[0x80, 0x80, 0x01])]
#[test_case(0x4321, &[0xa1, 0x86, 0x01])]
#[test_case(u64::MAX, &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01])]
fn from_into(u: u64, slice: &[u8]) {
    let fei = FlexIntEncoding::from(u);
    assert_eq!(fei.as_slice(), slice);

    let v = u64::try_from(fei).unwrap();
    assert_eq!(u, v);
}

// TODO: map errors to Strings, then verify string contents to distinguish too long vs overflow.
#[test_case(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01], Ok(u64::MAX))]
#[test_case(&[0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x02], Err("overflow"))]
#[test_case(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x01], Err("byte encoding too long"))]
fn try_from_overflow(slice: &[u8], expected: Result<u64, &str>) {
    let r = try_from_overflow_direct(slice).map_err(|e| e.to_string());

    match (r, expected) {
        (Ok(a), Ok(b)) => {
            assert_eq!(a, b);
        }
        (Err(s), Err(prefix)) => {
            assert!(
                s.starts_with(prefix),
                "missing prefix {prefix:?} in {s:?}"
            );
        }
        (Err(s), Ok(v)) => {
            panic!("expected Ok({v:?}), found Err({s:?})");
        }
        (Ok(v), Err(_)) => {
            panic!("expected Err(_), found Ok({v:?})");
        }
    }
}

fn try_from_overflow_direct(slice: &[u8]) -> anyhow::Result<u64> {
    let fei = FlexIntEncoding::try_from(slice)?;
    let u = u64::try_from(fei)?;
    Ok(u)
}
