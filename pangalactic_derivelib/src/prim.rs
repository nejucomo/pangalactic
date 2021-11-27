/// Type aliases to avoid type-confusion in low-level bindings. Call sites should use mod-name
/// prefixing to avoid confusion between these primitive "typeless" i64s versus the high-level type
/// wrappers. Ex: `prim::Link` != `Link` != `LinkHandle`.

pub type LinkKind = i64;
pub type LinkHandle = i64;
pub type BufReaderHandle = i64;
