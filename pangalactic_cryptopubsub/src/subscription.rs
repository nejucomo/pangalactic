use crate::Publisher;
use rust_sodium::crypto::{secretbox, sign};

#[derive(Clone, Debug)]
pub struct Subscription {
    signpub: sign::PublicKey,
    sboxkey: secretbox::Key,
}

impl From<Publisher> for Subscription {
    fn from(p: Publisher) -> Subscription {
        Subscription {
            signpub: p.signpair.public,
            sboxkey: p.sboxkey,
        }
    }
}

#[cfg(test)]
impl Subscription {
    pub(crate) fn expose_innards(&self) -> (&sign::PublicKey, &secretbox::Key) {
        (&self.signpub, &self.sboxkey)
    }
}
