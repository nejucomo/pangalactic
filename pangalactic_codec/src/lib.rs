pub mod b64;
mod codec;

pub use codec::{decode_bytes, decode_string, encode_bytes, encode_string, DecodeStringError};
