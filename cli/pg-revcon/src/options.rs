use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// pg revision control
#[derive(Debug, Parser)]
pub struct Options {
    #[clap(subcommand)]
    pub command: Command,
}

/// Revision Control commands
#[derive(Debug, Subcommand)]
pub enum Command {
    Info(InfoOptions),
    Init(InitOptions),
}

/// Repository info
#[derive(Debug, Args)]
pub struct InfoOptions {
    #[command(subcommand)]
    pub detail: Option<InfoDetail>,
}

/// Revision Control Info subcommands
#[derive(Debug, Subcommand)]
pub enum InfoDetail {
    Path(InfoPathOptions),
}

impl Default for InfoDetail {
    fn default() -> Self {
        InfoDetail::Path(InfoPathOptions::default())
    }
}

/// Print the control directory path
#[derive(Default, Debug, Args)]
pub struct InfoPathOptions {}

/// Initialize revision control
#[derive(Debug, Args)]
pub struct InitOptions {
    /// The workdir to initialize
    #[clap(long, short, default_value = ".")]
    pub workdir: PathBuf,
}
