pub fn encode<T>(obj: &T) -> String
where
    T: serde::Serialize,
{
    let bytes = serde_cbor::ser::to_vec_packed(obj).unwrap();
    base64::encode_config(bytes, base64::URL_SAFE_NO_PAD)
}
