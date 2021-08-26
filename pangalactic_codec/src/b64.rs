pub fn encode<T>(bytes: T) -> String
where
    T: AsRef<[u8]>,
{
    base64::encode_config(bytes, base64::URL_SAFE_NO_PAD)
}

pub fn decode<T>(input: T) -> Result<Vec<u8>, base64::DecodeError>
where
    T: AsRef<[u8]>,
{
    base64::decode_config(input, base64::URL_SAFE_NO_PAD)
}
