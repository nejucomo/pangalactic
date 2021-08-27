use crate::Link;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry<K> {
    pub name: String,
    pub link: Link<K>,
}
