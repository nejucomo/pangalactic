use pangalactic_primitives::{self as prim, LINK_KIND_DIR, LINK_KIND_FILE};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkKind {
    File,
    Dir,
}

impl TryFrom<prim::LinkKind> for LinkKind {
    type Error = String;

    fn try_from(u: prim::LinkKind) -> Result<Self, Self::Error> {
        use LinkKind::*;

        match u {
            LINK_KIND_FILE => Ok(File),
            LINK_KIND_DIR => Ok(Dir),
            _ => Err(format!("invalid LinkKind encoding {u:?}")),
        }
    }
}

impl From<LinkKind> for prim::LinkKind {
    fn from(lk: LinkKind) -> prim::LinkKind {
        use LinkKind::*;

        match lk {
            File => LINK_KIND_FILE,
            Dir => LINK_KIND_DIR,
        }
    }
}

impl FromStr for LinkKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        use LinkKind::*;

        match s {
            "file" => Ok(File),
            "dir" => Ok(Dir),
            other => Err(anyhow::anyhow!("unrecognized LinkKind {other:?}")),
        }
    }
}

impl fmt::Display for LinkKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LinkKind::*;

        match self {
            File => "file",
            Dir => "dir",
        }
        .fmt(f)
    }
}
