use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// low-level pubsub operations
#[derive(Clone, Debug, Parser)]
pub struct Options {
    #[clap(subcommand)]
    pub command: Command,
}

/// Lower-level pubsub commands
#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    Generate(GenerateOptions),
}

/// Generate a new pubcap, printing the subcap on stdout
#[derive(Clone, Debug, Args)]
pub struct GenerateOptions {
    #[clap(flatten)]
    pub pubcapopts: PubcapOptions,
}

/// Common options for using a pubcap
#[derive(Clone, Debug, Args)]
pub struct PubcapOptions {
    /// The path to the pubcap file
    #[clap(short, long)]
    pub pubcap: PathBuf,
}
