mod link;

pub use self::link::Link;

#[cfg(any(test, feature = "testutil"))]
pub mod testutil;

#[cfg(test)]
mod tests;
