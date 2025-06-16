use base64::engine::Engine;
pub use base64::DecodeError;

pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input)
}

pub fn encode_to_string<T: AsRef<[u8]>>(input: T, out: &mut String) {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode_string(input, out)
}

pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(input)
}
