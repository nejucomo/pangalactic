use pangalactic_hash::Hash;
use pangalactic_store::StoreCid;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DirDbCid(pub(crate) Hash);

impl StoreCid for DirDbCid {
    const SCHEME: &'static str = "dirdb";
}

impl fmt::Display for DirDbCid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for DirDbCid {
    type Err = <Hash as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Hash::from_str(s).map(DirDbCid)
    }
}
