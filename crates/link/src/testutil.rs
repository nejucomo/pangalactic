use std::fmt;

use pangalactic_cid::ContentIdentifier;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct FakeKey;

impl ContentIdentifier for FakeKey {}

impl fmt::Display for FakeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<FakeKey>")
    }
}

impl std::str::FromStr for FakeKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "<FakeKey>" {
            Ok(FakeKey)
        } else {
            anyhow::bail!("unexpected: {s:?}")
        }
    }
}
