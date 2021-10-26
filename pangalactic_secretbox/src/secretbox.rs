use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::secretbox;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SecretBoxKey(secretbox::Key);

#[derive(Debug, derive_more::From)]
pub enum OpenError {
    MalformedCiphertext,
    MalformedPlaintext(pangalactic_codec::DecodeBytesError),
}

impl SecretBoxKey {
    pub fn generate() -> SecretBoxKey {
        pangalactic_sodiuminit::init_if_necessary();
        SecretBoxKey(secretbox::gen_key())
    }

    pub fn seal<T>(&self, plaintext: T) -> Vec<u8>
    where
        T: AsRef<[u8]>,
    {
        use pangalactic_codec::encode_bytes;

        pangalactic_sodiuminit::init_if_necessary();
        let nonce = secretbox::gen_nonce();
        let ciphertext = secretbox::seal(plaintext.as_ref(), &nonce, &self.0);
        let unsealed = NonceTracker { nonce, ciphertext };
        encode_bytes(&unsealed)
    }

    pub fn open<T>(&self, ciphertext: T) -> Result<Vec<u8>, OpenError>
    where
        T: AsRef<[u8]>,
    {
        use pangalactic_codec::decode_bytes;

        let NonceTracker { nonce, ciphertext } = decode_bytes(ciphertext.as_ref())?;
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
