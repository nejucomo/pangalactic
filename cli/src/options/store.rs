use crate::dagops::{AnyPathDo, LinkDo};
use clap::{Args, Subcommand};
use pangalactic_dir::Name;

/// Interact directly with the store
#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    File(FileCommand),
    #[command(subcommand)]
    Dir(DirCommand),
    Copy(CopyOptions),
}

/// Low-level file operations
#[derive(Debug, Subcommand)]
pub enum FileCommand {
    /// Insert the file on stdin and print its key on stdout
    Put,
    /// Send the given file to stdout
    Get(FileGetOptions),
}

/// Low-level dir operations
#[derive(Debug, Subcommand)]
pub enum DirCommand {
    /// Print the link for the empty directory
    Empty,
    Link(DirLinkOptions),
    Unlink(DirUnlinkOptions),
    List(DirListOptions),
}

/// Set a link within a directory
#[derive(Debug, Args)]
pub struct DirLinkOptions {
    /// The directory to insert a link into
    pub dir: LinkDo,
    /// The name of the link entry in `dir`
    pub name: Name,
    /// The referent of the link entry in `dir`
    pub target: LinkDo,
}

/// Unlink a directory entry
#[derive(Debug, Args)]
pub struct DirUnlinkOptions {
    /// The directory to unlink an entry from
    pub dir: LinkDo,
    /// The name of the link entry in `dir`
    pub name: Name,
}

/// List a directory's contents
#[derive(Debug, Args)]
pub struct DirListOptions {
    /// The directory to list
    pub dir: LinkDo,
}

#[derive(Debug, Args)]
pub struct FileGetOptions {
    /// The link to get
    pub link: LinkDo,
}

/// Copy files or directories within or across store or host
#[derive(Debug, Args)]
pub struct CopyOptions {
    /// The source path
    pub source: AnyPathDo,
    /// The destination path
    pub dest: AnyPathDo,
}
