use serde::Serialize;

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
