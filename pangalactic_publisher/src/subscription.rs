use crate::Publisher;
use rust_sodium::crypto::{secretbox, sign};

#[derive(Debug)]
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
