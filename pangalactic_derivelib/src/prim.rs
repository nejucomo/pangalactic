/// Type aliases to avoid type-confusion in low-level bindings. Call sites should use mod-name
/// prefixing to avoid confusion between these primitive "typeless" i64s versus the high-level type
/// wrappers. Ex: `prim::Link` != `Link` != `LinkHandle`.

pub(crate) type Bool = i64;
pub(crate) type BufReaderHandle = i64;
pub(crate) type BufWriterHandle = i64;
pub(crate) type LinkHandle = i64;
pub(crate) type LinkKind = i64;
pub(crate) type MemLen = i64;
pub(crate) type ReadPtr = i64;

pub(crate) fn bool_host2guest(b: Bool) -> bool {
    match b {
        0 => false,
        1 => true,
        _ => unreachable!(),
    }
}
