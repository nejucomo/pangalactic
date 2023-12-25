use crate::dagops::LinkDo;
use clap::{Args, Subcommand};

/// Low-level file operations
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Insert the file on stdin and print its key on stdout
    Put,
    /// Send the given file to stdout
    Get(GetOptions),
}

#[derive(Debug, Args)]
pub struct GetOptions {
    /// The link to get
    pub link: LinkDo,
}
