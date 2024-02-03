mod dest;
mod source;

pub use self::dest::Destination;
pub use self::source::Source;

use crate::cmd;
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

    pub async fn run(self) -> anyhow::Result<()> {
        use Command::*;
        use StoreCommand::*;

        match self.command.unwrap() {
            Store(Put) => cmd::store_put().await,
            Store(Get(opts)) => cmd::store_get(&opts.link).await,
            Store(Xfer(XferOptions { source, dest })) => {
                dbg!(source);
                dbg!(dest);
                todo!()
            }
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
    Xfer(XferOptions),
}

/// Send the given file to stdout
#[derive(Debug, Args)]
pub struct StoreGetOptions {
    /// The link to get
    pub link: CliLink,
}

/// Transfer from SOURCE to DEST
#[derive(Debug, Args)]
pub struct XferOptions {
    pub source: Source,
    pub dest: Destination,
}
