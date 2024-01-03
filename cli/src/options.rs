mod dest;
mod source;

pub use self::dest::Destination;
pub use self::source::Source;

use crate::cmd::{Commander, Link};
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

    pub async fn run(self) -> anyhow::Result<()> {
        use Command::*;
        use StoreCommand::*;

        let mut cmdr = Commander::default();

        match self.command.unwrap() {
            Store(Put) => cmdr.store_put().await,
            Store(Get(StoreGetOptions { link })) => cmdr.store_get(&link).await,
            Store(Xfer(StoreXferOptions { source, dest })) => cmdr.store_xfer(&source, &dest).await,
        }
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
    /// Insert the file on stdin and print its key on stdout
    Put,
    Get(StoreGetOptions),
    Xfer(StoreXferOptions),
}

/// Send the given file to stdout
#[derive(Debug, Args)]
pub struct StoreGetOptions {
    /// The link to get
    pub link: Link,
}

/// Send the given file to stdout
#[derive(Debug, Args)]
pub struct StoreXferOptions {
    pub source: Source,
    pub dest: Destination,
}
