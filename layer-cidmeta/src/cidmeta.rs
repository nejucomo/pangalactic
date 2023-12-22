use pangalactic_store::Store;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CidMeta<S>
where
    S: Store,
{
    #[serde(bound(deserialize = "S:", serialize = "S:"))]
    pub(crate) cid: <S as Store>::CID,
    pub(crate) node_size: u64,
}

impl<S> CidMeta<S>
where
    S: Store,
{
    pub fn node_size(&self) -> u64 {
        self.node_size
    }
}

impl<S> PartialEq for CidMeta<S>
where
    S: Store,
{
    fn eq(&self, other: &Self) -> bool {
        (self.cid == other.cid) && (self.node_size == other.node_size)
    }
}

impl<S> Eq for CidMeta<S> where S: Store {}

impl<S> Clone for CidMeta<S>
where
    S: Store,
{
    fn clone(&self) -> Self {
        CidMeta {
            cid: self.cid.clone(),
            node_size: self.node_size,
        }
    }
}
