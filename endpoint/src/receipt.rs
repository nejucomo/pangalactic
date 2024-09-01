use std::fmt;
use std::path::PathBuf;

use pangalactic_linkpath::LinkPath;
use serde::Serialize;

/// Indicates the `CID` of a destination
///
/// This is isometric to `SourceEndpoint` except we replace `Stdin` with `Stdout` and the `Display` for `Stdout` is `""` and there is no `FromStr`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Receipt<C>
where
    C: Serialize,
{
    Stdout,
    Host(PathBuf),
    Store(LinkPath<C>),
}

impl<C> fmt::Display for Receipt<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Receipt::*;

        match self {
            Stdout => Ok(()),
            Host(pb) => pb.display().fmt(f),
            Store(sp) => sp.fmt(f),
        }
    }
}
