use crate::Kind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Link<K> {
    pub kind: Kind,
    pub key: K,
}
