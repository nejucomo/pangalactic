use ed25519_dalek::VerifyingKey;

#[derive(Debug)]
pub struct Subscription {
    #[allow(dead_code)]
    vk: VerifyingKey,
}

impl Subscription {
    pub(crate) fn from_vk(vk: VerifyingKey) -> Self {
        Subscription { vk }
    }
}
