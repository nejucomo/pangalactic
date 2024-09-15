pub mod aliases;
mod destination;
mod endpoint;
mod hostorstore;
mod hostpath;
mod origin;
mod stdio;

pub use self::aliases::{DestinationEndpoint, OriginEndpoint, Receipt};
pub use self::endpoint::Endpoint;
pub use self::hostorstore::HostOrStore;
pub use self::hostpath::HostPath;
pub use self::stdio::{Stdin, Stdio};

#[cfg(test)]
mod tests;
