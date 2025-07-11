use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// low-level pubsub operations
#[derive(Clone, Debug, Parser)]
pub struct Options {
    /// A subcommand
    #[clap(subcommand)]
    pub command: Command,
}

/// Lower-level pubsub commands
#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Operations on a publish-cap
    PubCap(PubCapOptions),
}

/// Generate a new pubcap, printing the subcap on stdout
#[derive(Clone, Debug, Args)]
pub struct PubCapOptions {
    /// The directory to store the new publish-cap file in
    #[clap(short, long, default_value = ".pg/PRIVLOCAL/")]
    pub pubcap_dir: PathBuf,

    /// The directory to store the new publish-cap file in
    #[clap(subcommand)]
    pub command: PubCapCommand,
}

/// Lower-level pubsub commands
#[derive(Clone, Debug, Subcommand)]
pub enum PubCapCommand {
    Generate,
    GetSubscribeCap,
}
