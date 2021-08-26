use crate::sekbox::SEKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReadCap<K> {
    pub(crate) basekey: K,
    pub(crate) sekey: SEKey,
}

impl<K: PartialEq> PartialEq for ReadCap<K> {
    fn eq(&self, other: &ReadCap<K>) -> bool {
        self.basekey == other.basekey && self.sekey == other.sekey
    }
}

impl<K: Eq> Eq for ReadCap<K> {}

use std::fmt::{Debug, Formatter, Result};

impl<K> Debug for ReadCap<K>
where
    K: Serialize,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ReadCap<{}>", pangalactic_codec::encode_string(&self))
    }
}
