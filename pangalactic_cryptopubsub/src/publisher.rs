use pangalactic_secretbox::SecretBoxKey;
use pangalactic_signpair::SigningPair;

#[derive(Clone, Debug)]
pub struct Publisher {
    pub(crate) signpair: SigningPair,
    pub(crate) sboxkey: SecretBoxKey,
}

impl Publisher {
    pub fn generate() -> Publisher {
        let signpair = SigningPair::generate();
        let sboxkey = SecretBoxKey::generate();
        Publisher { signpair, sboxkey }
    }
}
