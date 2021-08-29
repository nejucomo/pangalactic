mod error;

pub use self::error::{DecodeBytesError, DecodeStringError};
use serde::{de::DeserializeOwned, Deserialize};

pub fn decode_bytes<'a, T>(slice: &'a [u8]) -> Result<T, DecodeBytesError>
where
    T: Deserialize<'a>,
{
    let x = serde_cbor::from_slice(slice)?;
    Ok(x)
}

pub fn decode_string<I, O>(input: I) -> Result<O, DecodeStringError>
where
    I: AsRef<[u8]>,
    O: DeserializeOwned,
{
    let bytes = crate::b64::decode(input)?;
    let output = decode_bytes(&bytes[..])?;
    Ok(output)
}
