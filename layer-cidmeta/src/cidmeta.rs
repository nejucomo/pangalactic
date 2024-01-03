use std::{fmt::Display, str::FromStr};

use pangalactic_store::{Store, StoreCid};
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

impl<S> StoreCid for CidMeta<S> where S: Store {}

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

impl<S> PartialEq for CidMeta<S>
where
    S: Store,
{
    fn eq(&self, other: &Self) -> bool {
        (self.cid == other.cid) && (self.node_size == other.node_size)
    }
}

impl<S> Eq for CidMeta<S> where S: Store {}

impl<S> FromStr for CidMeta<S>
where
    S: Store,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use pangalactic_b64 as b64;
        use pangalactic_serialization::deserialize;

        let (cidstr, metastr) = s
            .split_once('-')
            .ok_or_else(|| anyhow::anyhow!("expected '-'"))?;

        let cid = cidstr.parse()?;
        let node_size = deserialize(&b64::decode(metastr)?)?;
        Ok(CidMeta { cid, node_size })
    }
}

impl<S> Display for CidMeta<S>
where
    S: Store,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use pangalactic_b64 as b64;
        use pangalactic_serialization::serialize;

        let metastr = b64::encode(serialize(&self.node_size).unwrap());
        write!(f, "{}-{}", &self.cid, metastr)
    }
}
