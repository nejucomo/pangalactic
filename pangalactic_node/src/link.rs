use crate::Kind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Link<K> {
    pub kind: Kind,
    pub key: K,
}
