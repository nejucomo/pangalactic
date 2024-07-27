mod dest;
mod source;

pub use self::dest::Destination;
pub use self::source::Source;

use crate::store::CliLink;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    pub command: Option<Command>,
}

impl Options {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Store(StoreCommand),
}

/// Interact directly with the store
#[derive(Debug, Subcommand)]
pub enum StoreCommand {
    Put(StorePutOptions),
    Get(StoreGetOptions),
    Xfer(StoreXferOptions),
}

/// Insert the file on stdin and print its key on stdout
#[derive(Debug, Args)]
pub struct StorePutOptions {}

/// Send the given file to stdout
#[derive(Debug, Args)]
pub struct StoreGetOptions {
    /// The link to get
    pub link: CliLink,
}

/// Transfer from SOURCE to DEST
#[derive(Debug, Args)]
pub struct StoreXferOptions {
    pub source: Source,
    pub dest: Destination,
}
