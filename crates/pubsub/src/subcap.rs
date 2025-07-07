use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeCap {
    vk: VerifyingKey,
}

impl SubscribeCap {
    pub(crate) fn from_vk(vk: VerifyingKey) -> Self {
        SubscribeCap { vk }
    }

    pub(crate) fn verify(&self, payload: &[u8], sig: &Signature) -> anyhow::Result<()> {
        self.vk.verify(payload, sig)?;
        Ok(())
    }
}
