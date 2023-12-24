use pangalactic_hash::Hash;
use pangalactic_store::StoreCid;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct MemCid(pub(crate) Hash);

impl StoreCid for MemCid {
    const SCHEME: &'static str = "mem";
}

impl fmt::Display for MemCid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for MemCid {
    type Err = <Hash as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Hash::from_str(s).map(MemCid)
    }
}
