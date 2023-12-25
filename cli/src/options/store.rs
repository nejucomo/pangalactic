pub mod dir;
pub mod file;
pub mod tree;

use crate::dagops::AnyPathDo;
use clap::{Args, Subcommand};

/// Interact directly with the store
#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    File(self::file::Command),
    #[command(subcommand)]
    Dir(self::dir::Command),
    #[command(subcommand)]
    Tree(self::tree::Command),
    Copy(CopyOptions),
}

/// Copy files or directories within or across store or host
#[derive(Debug, Args)]
pub struct CopyOptions {
    /// The source path
    pub source: AnyPathDo,
    /// The destination path
    pub dest: AnyPathDo,
}
