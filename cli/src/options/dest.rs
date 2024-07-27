use crate::store::CliStoreDestination;
use pangalactic_link::SCHEME_PREFIX;
use std::{fmt::Display, path::PathBuf, str::FromStr};

#[derive(Clone, Debug)]
pub enum Destination {
    Stdout,
    Host(PathBuf),
    Store(Option<CliStoreDestination>),
}
use Destination::*;

impl Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stdout => '-'.fmt(f),
            Host(pb) => pb.display().fmt(f),
            Store(None) => SCHEME_PREFIX.fmt(f),
            Store(Some(sp)) => sp.fmt(f),
        }
    }
}

impl FromStr for Destination {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            Ok(Stdout)
        } else if s == SCHEME_PREFIX {
            Ok(Store(None))
        } else if s.starts_with(SCHEME_PREFIX) {
            s.parse().map(Some).map(Store)
        } else {
            s.parse().map(Host).map_err(anyhow::Error::from)
        }
    }
}
