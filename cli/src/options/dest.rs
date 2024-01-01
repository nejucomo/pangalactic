use std::{fmt::Display, path::PathBuf, str::FromStr};

use pangalactic_storepath::StoreDestination;

use crate::store::CliCid;

#[derive(Clone, Debug)]
pub enum Destination {
    Stdout,
    Host(PathBuf),
    StoreScheme,
    Store(StoreDestination<CliCid>),
}
use Destination::*;

impl Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stdout => '-'.fmt(f),
            Host(pb) => pb.display().fmt(f),
            StoreScheme => unimplemented!("BUG: Change link encoding to be URL-like"),
            Store(sp) => sp.fmt(f),
        }
    }
}

impl FromStr for Destination {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!("BUG: Change link encoding to be URL-like: {s:?}")
    }
}
