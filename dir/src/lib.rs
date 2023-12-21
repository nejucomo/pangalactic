mod dir;

// TODO: newtype String which excludes illegal names:
pub type Name = String;
pub type NameRef = str;

pub use self::dir::Directory;
