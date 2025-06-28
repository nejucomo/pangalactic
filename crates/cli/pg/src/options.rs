use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use pangalactic_store_dirdb::DirDbStore;

/// pg revision control
#[derive(Debug, Parser)]
pub struct Options {
    /// The path to the dirdb store directory
    #[clap(short, long, default_value_t)]
    pub dirdb: DirDbStore,

    #[clap(subcommand)]
    pub command: Command,
}

/// Revision Control commands
#[derive(Debug, Subcommand)]
pub enum Command {
    Info(InfoOptions),
    Init(InitOptions),
}

impl Default for Command {
    fn default() -> Self {
        Command::Info(InfoOptions::default())
    }
}

/// Repository info
#[derive(Default, Debug, Args)]
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
