mod dir;

// TODO: newtype String which excludes illegal names:
pub type Name = String;
pub type NameRef = str;

pub use self::dir::Directory;

#[cfg(feature = "host")]
mod hostdir;

#[cfg(feature = "host")]
pub use self::hostdir::HostDirectory;

#[cfg(test)]
mod tests;
