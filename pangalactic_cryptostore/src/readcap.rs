use crate::sekbox::SEKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadCap<K> {
    pub(crate) basekey: K,
    pub(crate) sekey: SEKey,
}
