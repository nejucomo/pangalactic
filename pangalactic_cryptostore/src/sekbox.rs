use serde::{Deserialize, Serialize};
/// Single-Encryption Key box: Symmetric encryption confidentiality which assumes any given key is
/// used to encrypt exactly one message.
/// TODO: Replace w/ pangalactic_secretbox;
use sodiumoxide::crypto::secretbox;

pub(crate) const KEY_LENGTH: usize = secretbox::KEYBYTES;
pub(crate) type KeyBytes = [u8; KEY_LENGTH];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SEKey(secretbox::Key);

impl SEKey {
    pub(crate) fn seal(&self, plaintext: &[u8]) -> Vec<u8> {
        secretbox::seal(plaintext, &zero_nonce(), &self.0)
    }

    pub(crate) fn unseal(&self, ciphertext: &[u8]) -> Result<Vec<u8>, ()> {
        secretbox::open(ciphertext, &zero_nonce(), &self.0)
    }
}

impl From<&KeyBytes> for SEKey {
    fn from(array: &KeyBytes) -> SEKey {
        SEKey(secretbox::Key::from_slice(&array[..]).unwrap())
    }
}

impl PartialEq for SEKey {
    fn eq(&self, other: &SEKey) -> bool {
        self.0 == other.0
    }
}

impl Eq for SEKey {}

fn zero_nonce() -> secretbox::Nonce {
    // TODO: document crypto design and 0 nonce:
    secretbox::Nonce::from_slice(&[0u8; secretbox::NONCEBYTES]).unwrap()
}
