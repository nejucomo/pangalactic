const TOKEN_BYTE_LENGTH: usize = 32;

pub fn generate() -> String {
    use rand::RngCore;

    let mut bytes = [0u8; TOKEN_BYTE_LENGTH];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    crate::b64::encode(&bytes)
}
