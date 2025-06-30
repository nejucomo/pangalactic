pub mod b64;
mod bytes;

pub use self::bytes::{deserialize, serialize};

#[cfg(feature = "testutil")]
mod testutil;

#[cfg(feature = "testutil")]
pub use self::testutil::check_serialize_then_deserialize_equality;
