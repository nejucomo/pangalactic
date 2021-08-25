use derive_more::{From, Into};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Eq, PartialEq, From, Into, Serialize, Deserialize)]
pub struct Key {
    ix: usize,
}

impl pangalactic_store::StoreKey for Key {}
