use crate::store::CliCid;
use pangalactic_storepath::StorePath;
use std::{fmt::Display, path::PathBuf, str::FromStr};

#[derive(Clone, Debug)]
pub enum Source {
    Stdin,
    Host(PathBuf),
    Store(StorePath<CliCid>),
}
use Source::*;

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stdin => '-'.fmt(f),
            Host(pb) => pb.display().fmt(f),
            Store(sp) => sp.fmt(f),
        }
    }
}

impl FromStr for Source {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            Ok(Stdin)
        } else {
            // BUG: The encoding should not require us to do a trial parse:
            match s.parse::<StorePath<_>>() {
                Ok(sp) => Ok(Store(sp)),
                Err(_) => {
                    let pb = s.parse::<PathBuf>()?;
                    Ok(Host(pb))
                }
            }
        }
    }
}
