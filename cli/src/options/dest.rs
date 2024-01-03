use crate::cmd::{Link, StoreDestination};
use std::{fmt::Display, path::PathBuf, str::FromStr};

#[derive(Clone, Debug)]
pub enum Destination {
    Stdout,
    Host(PathBuf),
    StoreScheme,
    Store(StoreDestination),
}
use Destination::*;

impl Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stdout => '-'.fmt(f),
            Host(pb) => pb.display().fmt(f),
            StoreScheme => Link::prefix().fmt(f),
            Store(sp) => sp.fmt(f),
        }
    }
}

impl FromStr for Destination {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prefix = Link::prefix();
        if s == "-" {
            Ok(Stdout)
        } else if s == prefix {
            Ok(StoreScheme)
        } else if s.starts_with(&prefix) {
            let sp = s.parse()?;
            Ok(Store(sp))
        } else {
            let pb = s.parse::<PathBuf>()?;
            Ok(Host(pb))
        }
    }
}
