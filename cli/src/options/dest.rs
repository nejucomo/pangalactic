use crate::cmd::{Link, StoreDestination};
use std::{fmt::Display, path::PathBuf, str::FromStr};

#[derive(Clone, Debug)]
pub enum Destination {
    Stdout,
    Host(PathBuf),
    /// If no destination is present, the inbound file/dir is inserted unlinked to any containing directory:
    Store(Option<StoreDestination>),
}
use Destination::*;

impl Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stdout => '-'.fmt(f),
            Host(pb) => pb.display().fmt(f),
            Store(osp) => {
                if let Some(sp) = osp {
                    sp.fmt(f)
                } else {
                    Link::SCHEME.fmt(f)
                }
            }
        }
    }
}

impl FromStr for Destination {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prefix = format!("{}:", Link::SCHEME);
        if s == "-" {
            Ok(Stdout)
        } else if s.starts_with(&prefix) {
            let osp = if s == prefix {
                None
            } else {
                s.parse().map(Some)?
            };
            Ok(Store(osp))
        } else {
            let pb = s.parse::<PathBuf>()?;
            Ok(Host(pb))
        }
    }
}
