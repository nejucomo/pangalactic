use rust_sodium::crypto::secretbox;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct SecretBoxKey(secretbox::Key);

#[derive(Debug, derive_more::From)]
pub enum OpenError {
    MalformedCiphertext,
    MalformedPlaintext(pangalactic_codec::DecodeBytesError),
}

impl SecretBoxKey {
    pub fn generate() -> SecretBoxKey {
        SecretBoxKey(secretbox::gen_key())
    }

    pub fn seal(&self, plaintext: &[u8]) -> Vec<u8> {
        use pangalactic_codec::encode_bytes;

        let nonce = secretbox::gen_nonce();
        let ciphertext = secretbox::seal(&plaintext, &nonce, &self.0);
        let unsealed = NonceTracker { nonce, ciphertext };
        encode_bytes(&unsealed)
    }

    pub fn open(&self, ciphertext: &[u8]) -> Result<Vec<u8>, OpenError> {
        use pangalactic_codec::decode_bytes;

        let NonceTracker { nonce, ciphertext } = decode_bytes(ciphertext)?;
        let plaintext = secretbox::open(&ciphertext[..], &nonce, &self.0)
            .map_err(|()| OpenError::MalformedCiphertext)?;

        Ok(plaintext)
    }
}

#[derive(Deserialize, Serialize)]
struct NonceTracker {
    nonce: secretbox::Nonce,
    ciphertext: Vec<u8>,
}
