use crate::{Distributor, Publication, PublicationContents};
use pangalactic_secretbox::{self as secretbox, SecretBoxKey};

#[derive(Clone, Debug)]
pub struct Subscriber {
    distributor: Distributor,
    sboxkey: SecretBoxKey,
}

#[derive(Debug, derive_more::From)]
pub enum UnwrapError {
    Distribution(crate::distributor::UnwrapError),
    SecretBox(secretbox::OpenError),
}

impl Subscriber {
    pub(crate) fn new(distributor: Distributor, sboxkey: SecretBoxKey) -> Subscriber {
        Subscriber {
            distributor,
            sboxkey,
        }
    }

    pub fn unwrap(&self, p: &Publication) -> Result<PublicationContents, UnwrapError> {
        let pc = self.distributor.unwrap(p)?;
        let plaintext = self.sboxkey.open(pc.data)?;
        Ok(PublicationContents {
            sequence: pc.sequence,
            data: plaintext,
        })
    }
}
