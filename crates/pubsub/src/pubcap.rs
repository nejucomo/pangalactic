use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};

use crate::SubscribeCap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishCap {
    sk: SigningKey,
}

impl PublishCap {
    pub fn generate<R>(mut r: R) -> Self
    where
        R: rand::CryptoRng,
    {
        use ed25519_dalek::{SecretKey, SECRET_KEY_LENGTH};

        let mut sk: SecretKey = [0; SECRET_KEY_LENGTH];
        r.fill_bytes(&mut sk);

        PublishCap {
            sk: SigningKey::from_bytes(&sk),
        }
    }

    pub fn subscribe_cap(&self) -> SubscribeCap {
        SubscribeCap::from_vk(self.sk.verifying_key())
    }
}
