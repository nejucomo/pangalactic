#![allow(async_fn_in_trait)]

mod anydest;
mod anysource;
mod linkdest;
mod linkpath;

pub use self::anydest::AnyDestination;
pub use self::anysource::AnySource;
pub use self::linkdest::LinkDestination;
pub use self::linkpath::LinkPath;
