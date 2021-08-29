use crate::{Distributor, Publication, PublicationContents, Subscriber};
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

    pub fn distributor(&self) -> Distributor {
        Distributor::from(self.signpair.verifier)
    }

    pub fn subscriber(&self) -> Subscriber {
        Subscriber::new(self.distributor(), self.sboxkey.clone())
    }

    pub fn publish(&self, sequence: u64, msg: &[u8]) -> Publication {
        use pangalactic_codec::encode_bytes;

        let pstate = PublicationContents {
            sequence,
            data: self.sboxkey.seal(msg),
        };
        let unsignedbytes = encode_bytes(&pstate);
        let sigbytes = self.signpair.signer.sign(&unsignedbytes[..]);
        Publication::from(sigbytes)
    }
}
