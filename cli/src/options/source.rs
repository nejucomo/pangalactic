use crate::cmd::{Link, StorePath};
use std::{fmt::Display, path::PathBuf, str::FromStr};

#[derive(Debug)]
pub enum Source {
    Stdin,
    Host(PathBuf),
    Store(StorePath),
}
use Source::*;

impl Clone for Source {
    fn clone(&self) -> Self {
        match self {
            Stdin => Stdin,
            Host(pb) => Host(pb.clone()),
            Store(sp) => Store(sp.clone()),
        }
    }
}

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
        } else if s.starts_with(&format!("{}:", Link::SCHEME)) {
            let sp = s.parse()?;
            Ok(Store(sp))
        } else {
            let pb = s.parse::<PathBuf>()?;
            Ok(Host(pb))
        }
    }
}
