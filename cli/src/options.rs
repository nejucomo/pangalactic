use crate::cmd;
use clap::{Parser, Subcommand};

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
            Store(Insert) => cmd::store_insert().await,
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
    Insert,
}
