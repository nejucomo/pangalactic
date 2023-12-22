use crate::cmd;
use crate::store::CliLink;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    command: Option<Command>,
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
    /// Send the given file to stdout
    Get(StoreGetOptions),
}

#[derive(Debug, Args)]
pub struct StoreGetOptions {
    /// The link to get
    link: CliLink,
}
