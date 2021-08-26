use crate::Dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Node<K> {
    File(Vec<u8>),
    Dir(Dir<K>),
}
