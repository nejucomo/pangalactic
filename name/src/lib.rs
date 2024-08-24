mod error;
mod name;
mod path;

pub const SEPARATOR: char = '/';

pub use self::error::{InvalidName, InvalidPath, NameError, PathError};
pub use self::name::{Name, NameRef};
pub use self::path::{Path, PathRef};
