mod link;

pub const SCHEME: &str = "pg";
pub const SCHEME_PREFIX: &str = "pg:";

pub use self::link::Link;

#[cfg(any(test, feature = "testutil"))]
pub mod testutil;

#[cfg(test)]
mod tests;
