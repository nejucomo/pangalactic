/// Type aliases to avoid type-confusion in low-level bindings. Call sites should use mod-name
/// prefixing to avoid confusion between these primitive "typeless" i64s versus the high-level type
/// wrappers. Ex: `prim::Link` != `Link` != `LinkHandle`.

pub(crate) type Bool = i64;
pub(crate) type BufReaderHandle = i64;
pub(crate) type BufWriterHandle = i64;
pub(crate) type DirWriterHandle = i64;
pub(crate) type LinkHandle = i64;
pub(crate) type LinkKind = i64;
pub(crate) type MemLen = i64;
pub(crate) type ReadPtr = i64;
pub(crate) type WritePtr = i64;

pub(crate) fn bool_host2guest(b: Bool) -> bool {
    match b {
        0 => false,
        1 => true,
        _ => unreachable!(),
    }
}

pub(crate) fn bytes_guest2host<B>(bytes: B) -> (ReadPtr, MemLen)
where
    B: AsRef<[u8]>,
{
    use std::convert::TryInto;

    let buf = bytes.as_ref();
    let bufptr = buf.as_ptr() as i64; // BUG: How to do this without overflow which would cause memory corruption?
    let buflenu: usize = buf.len();
    let buflen: MemLen = buflenu.try_into().unwrap();

    (bufptr, buflen)
}

pub(crate) fn bytes_guest2host_mut<B>(mut bytes: B) -> (WritePtr, MemLen)
where
    B: AsMut<[u8]>,
{
    use std::convert::TryInto;

    let buf = bytes.as_mut();
    let bufptr = buf.as_mut_ptr() as i64; // BUG: How to do this without overflow which would cause memory corruption?
    let buflenu: usize = buf.len();
    let buflen: MemLen = buflenu.try_into().unwrap();

    (bufptr, buflen)
}
