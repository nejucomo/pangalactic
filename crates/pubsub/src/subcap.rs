use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeCap {
    #[allow(dead_code)]
    vk: VerifyingKey,
}

impl SubscribeCap {
    pub(crate) fn from_vk(vk: VerifyingKey) -> Self {
        SubscribeCap { vk }
    }
}
