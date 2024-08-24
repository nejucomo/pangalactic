use std::{fmt, path::PathBuf, str::FromStr};

use pangalactic_link::SCHEME_PREFIX;
use serde::{de::DeserializeOwned, Serialize};

use crate::LinkDestination;

#[derive(Clone)]
pub enum AnyDestination<C> {
    Stdout,
    Host(PathBuf),
    Store(Option<LinkDestination<C>>),
}
use AnyDestination::*;

impl<C> fmt::Display for AnyDestination<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stdout => '-'.fmt(f),
            Host(pb) => pb.display().fmt(f),
            Store(None) => SCHEME_PREFIX.fmt(f),
            Store(Some(sp)) => sp.fmt(f),
        }
    }
}

impl<C> fmt::Debug for AnyDestination<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<C> FromStr for AnyDestination<C>
where
    C: DeserializeOwned,
{
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
