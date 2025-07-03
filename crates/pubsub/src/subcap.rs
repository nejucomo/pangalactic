use ed25519_dalek::VerifyingKey;

#[derive(Debug)]
pub struct SubscribeCap {
    #[allow(dead_code)]
    vk: VerifyingKey,
}

impl SubscribeCap {
    pub(crate) fn from_vk(vk: VerifyingKey) -> Self {
        SubscribeCap { vk }
    }
}
