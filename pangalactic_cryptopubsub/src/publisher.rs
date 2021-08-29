use crate::signingpair::SigningPair;
use rust_sodium::crypto::secretbox;

#[derive(Clone, Debug)]
pub struct Publisher {
    pub(crate) signpair: SigningPair,
    pub(crate) sboxkey: secretbox::Key,
}

impl Publisher {
    pub fn generate() -> Publisher {
        let signpair = SigningPair::generate();
        let sboxkey = secretbox::gen_key();
        Publisher { signpair, sboxkey }
    }
}
