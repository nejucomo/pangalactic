pub mod b64;
mod dec;
mod enc;

#[cfg(test)]
mod tests;

pub use dec::{decode_bytes, decode_string, DecodeBytesError, DecodeStringError};
pub use enc::{encode_bytes, encode_string};
