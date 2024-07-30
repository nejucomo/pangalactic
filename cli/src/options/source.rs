use std::{fmt::Display, path::PathBuf, str::FromStr};

use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::SCHEME_PREFIX;

#[derive(Debug)]
pub enum Source {
    Stdin,
    Host(PathBuf),
    Store(StorePath<CidMeta<<DirDbStore as pangalactic_store::Store>::CID>>),
}
use pangalactic_store_dirdb::DirDbStore;
use pangalactic_storepath::StorePath;
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
        } else if s.starts_with(SCHEME_PREFIX) {
            s.parse().map(Store)
        } else {
            s.parse().map(Host).map_err(anyhow::Error::from)
        }
    }
}
