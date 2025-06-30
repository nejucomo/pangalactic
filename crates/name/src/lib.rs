mod error;
mod fromutf8;
mod name;
mod nepath;
mod path;

pub const SEPARATOR: char = '/';

pub use self::error::{InvalidName, NameError};
pub(crate) use self::fromutf8::from_utf8;
pub use self::name::{Name, NameRef};
pub use self::nepath::{NonEmptyPath, NonEmptyPathRef};
pub use self::path::{Path, PathRef};
