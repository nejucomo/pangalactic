use ed25519_dalek::SigningKey;

use crate::Subscription;

#[derive(Debug)]
pub struct Publisher {
    sk: SigningKey,
}

impl Publisher {
    pub fn generate<R>(mut r: R) -> Self
    where
        R: rand_core::CryptoRng,
    {
        use ed25519_dalek::{SecretKey, SECRET_KEY_LENGTH};

        let mut sk: SecretKey = [0; SECRET_KEY_LENGTH];
        r.fill_bytes(&mut sk);

        Publisher {
            sk: SigningKey::from_bytes(&sk),
        }
    }

    pub fn subscription(&self) -> Subscription {
        Subscription::from_vk(self.sk.verifying_key())
    }
}
