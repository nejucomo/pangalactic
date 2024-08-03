use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use pangalactic_cid::ContentIdentifier;

use pangalactic_serialization::b64;
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

const STRINGIFIED_PREFIX: &'static str = "rawcidmeta";

impl<C> Display for CidMeta<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}",
            STRINGIFIED_PREFIX,
            b64::serialize(self).unwrap()
        )
    }
}

impl<C> FromStr for CidMeta<C>
where
    C: DeserializeOwned,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use pangalactic_chomper::Chomper;

        let mut ch = Chomper::from(s);
        ch.require_prefix(":", STRINGIFIED_PREFIX)?;
        b64::deserialize(ch)
    }
}
