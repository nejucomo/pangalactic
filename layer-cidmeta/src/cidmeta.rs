use pangalactic_store::Store;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

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

impl<S> FromStr for CidMeta<S>
where
    S: Store,
    <S as Store>::CID: FromStr<Err = anyhow::Error>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (cidtext, sizetext) = s
            .split_once(':')
            .ok_or_else(|| anyhow::anyhow!("missing ':' in {s:?}"))?;
        let cid = cidtext.parse()?;
        let node_size = sizetext.parse()?;
        Ok(CidMeta { cid, node_size })
    }
}

impl<S> fmt::Display for CidMeta<S>
where
    S: Store,
    <S as Store>::CID: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.cid.fmt(f)?;
        ':'.fmt(f)?;
        self.node_size.fmt(f)?;
        Ok(())
    }
}
