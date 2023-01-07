//! Primitive types shared across the dagwasm host/guest boundary.

pub type HandleLink = u64;
pub type HandleByteReader = u64;
pub type HandleByteWriter = u64;
pub type HandleDirReader = u64;
pub type PtrRead = u64;
pub type PtrWrite = u64;
pub type ByteLen = u64;

pub type LinkKind = u64;
pub const LINK_KIND_FILE: LinkKind = 0;
pub const LINK_KIND_DIR: LinkKind = 1;

pub type Bool = u64;
pub const FALSE: Bool = 0;
pub const TRUE: Bool = 1;
