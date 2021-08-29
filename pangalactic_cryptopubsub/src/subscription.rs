use crate::Publisher;
use pangalactic_secretbox::SecretBoxKey;
use pangalactic_signpair::Verifier;

#[derive(Clone, Debug)]
pub struct Subscription {
    verifier: Verifier,
    sboxkey: SecretBoxKey,
}

impl From<Publisher> for Subscription {
    fn from(p: Publisher) -> Subscription {
        Subscription {
            verifier: p.signpair.verifier,
            sboxkey: p.sboxkey,
        }
    }
}

#[cfg(test)]
impl Subscription {
    pub(crate) fn expose_innards(&self) -> (&Verifier, &SecretBoxKey) {
        (&self.verifier, &self.sboxkey)
    }
}
