use crate::sekbox::SEKey;
use pangalactic_store::StoreKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReadCap<K> {
    pub(crate) basekey: K,
    pub(crate) sekey: SEKey,
}

impl<K: StoreKey> StoreKey for ReadCap<K> {}

impl<K: PartialEq> PartialEq for ReadCap<K> {
    fn eq(&self, other: &ReadCap<K>) -> bool {
        self.basekey == other.basekey && self.sekey == other.sekey
    }
}

impl<K: Eq> Eq for ReadCap<K> {}
