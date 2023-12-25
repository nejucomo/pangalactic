use crate::dagops::LinkDo;
use clap::{Args, Subcommand};
use pangalactic_dir::Name;

/// Low-level dir operations
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Print the link for the empty directory
    Empty,
    Link(LinkOptions),
    Unlink(UnlinkOptions),
    List(ListOptions),
}

/// Set a link within a directory
#[derive(Debug, Args)]
pub struct LinkOptions {
    /// The directory to insert a link into
    pub dir: LinkDo,
    /// The name of the link entry in `dir`
    pub name: Name,
    /// The referent of the link entry in `dir`
    pub target: LinkDo,
}

/// Unlink a directory entry
#[derive(Debug, Args)]
pub struct UnlinkOptions {
    /// The directory to unlink an entry from
    pub dir: LinkDo,
    /// The name of the link entry in `dir`
    pub name: Name,
}

/// List a directory's contents
#[derive(Debug, Args)]
pub struct ListOptions {
    /// The directory to list
    pub dir: LinkDo,
}
