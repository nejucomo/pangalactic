use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub fn encode_bytes<T>(x: &T) -> Vec<u8>
where
    T: Serialize,
{
    serde_cbor::ser::to_vec_packed(x).unwrap()
}

pub fn encode_string<T>(x: &T) -> String
where
    T: Serialize,
{
    crate::b64::encode(encode_bytes(x))
}

pub fn decode_bytes<'a, T>(slice: &'a [u8]) -> serde_cbor::Result<T>
where
    T: Deserialize<'a>,
{
    serde_cbor::from_slice(slice)
}

#[derive(Debug, derive_more::From)]
pub enum DecodeStringError {
    Base64(base64::DecodeError),
    Cbor(serde_cbor::Error),
}

pub fn decode_string<I, O>(input: I) -> Result<O, DecodeStringError>
where
    I: AsRef<[u8]>,
    O: DeserializeOwned,
{
    let bytes = crate::b64::decode(input)?;
    let output = serde_cbor::from_slice(&bytes[..])?;
    Ok(output)
}
