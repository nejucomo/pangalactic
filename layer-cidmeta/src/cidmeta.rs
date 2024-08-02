use std::fmt::Debug;

use pangalactic_cid::ContentIdentifier;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, derive_more::Constructor)]
pub struct CidMeta<C> {
    pub(crate) cid: C,
    pub(crate) node_size: u64,
}

impl<C> CidMeta<C> {
    pub fn node_size(&self) -> u64 {
        self.node_size
    }
}

impl<C> ContentIdentifier for CidMeta<C> where
    C: Clone + Debug + Eq + Send + Sync + Serialize + DeserializeOwned
{
}
