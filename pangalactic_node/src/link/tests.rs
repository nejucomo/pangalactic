use crate::{
    Kind::{self, *},
    Link,
};
use test_case::test_case;

#[test_case(File, File)]
#[test_case(File, Dir)]
#[test_case(Dir, File)]
#[test_case(Dir, Dir)]
fn test_get_key(linkkind: Kind, getkind: Kind) {
    let (inkey, link) = make_link(linkkind);
    match link.get_key(getkind) {
        Ok(&outkey) => {
            assert_eq!(linkkind, getkind);
            assert_eq!(inkey, outkey);
        }
        Err(_) => {
            assert_ne!(linkkind, getkind);
        }
    }
}

#[test_case(File)]
#[test_case(Dir)]
fn test_get_file_key(linkkind: Kind) {
    let (inkey, link) = make_link(linkkind);
    match link.get_file_key() {
        Ok(&outkey) => {
            assert_eq!(linkkind, File);
            assert_eq!(inkey, outkey);
        }
        Err(_) => {
            assert_eq!(linkkind, Dir);
        }
    }
}

#[test_case(File)]
#[test_case(Dir)]
fn test_get_dir_key(linkkind: Kind) {
    let (inkey, link) = make_link(linkkind);
    match link.get_dir_key() {
        Ok(&outkey) => {
            assert_eq!(linkkind, Dir);
            assert_eq!(inkey, outkey);
        }
        Err(_) => {
            assert_eq!(linkkind, File);
        }
    }
}

fn make_link(linkkind: Kind) -> (&'static [u8], Link<&'static [u8]>) {
    let inkey = &b"fake key"[..];
    let link = Link {
        kind: linkkind,
        key: inkey,
    };
    (inkey, link)
}
