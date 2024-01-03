use crate::cmd::StorePath;
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
        } else {
            // BUG: The encoding should not require us to do a trial parse:
            match s.parse::<StorePath>() {
                Ok(sp) => Ok(Store(sp)),
                Err(_) => {
                    let pb = s.parse::<PathBuf>()?;
                    Ok(Host(pb))
                }
            }
        }
    }
}
